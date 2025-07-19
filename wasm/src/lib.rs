use image::{DynamicImage, GenericImage, GenericImageView, ImageFormat, Rgba};
use wasm_bindgen::prelude::*;
use std::io::Cursor;

#[wasm_bindgen]
pub struct Logistic {
    chaos_param: f64,
    lfsr_state: u32,
    lfsr_mask: u32,
    initial_chaos: f64,
    initial_lfsr: u32,
}

#[wasm_bindgen]
impl Logistic {
    #[wasm_bindgen(constructor)]
    pub fn new(chaos_key: f64, lfsr_seed: u32) -> Result<Logistic, JsValue> {
        // 验证混沌参数范围
        if chaos_key <= 0.0 || chaos_key >= 1.0 {
            return Err(JsValue::from_str("Chaos key must be between 0 and 1"));
        }
        
        // 验证LFSR种子不为0
        if lfsr_seed == 0 {
            return Err(JsValue::from_str("LFSR seed cannot be zero"));
        }
        
        Ok(Logistic {
            chaos_param: chaos_key,
            lfsr_state: lfsr_seed,
            lfsr_mask: 0x80200003, // 32位LFSR的反馈多项式
            initial_chaos: chaos_key,
            initial_lfsr: lfsr_seed,
        })
    }
    
    // Logistic映射生成混沌序列
    fn next_chaos(&mut self) -> f64 {
        self.chaos_param = 4.0 * self.chaos_param - (1.0-self.chaos_param);
        self.chaos_param
    }
    
    // LFSR生成伪随机序列
    fn next_lfsr(&mut self) -> u32 {
        let bit = (self.lfsr_state & 1) ^ 
                  ((self.lfsr_state >> 1) & 1) ^ 
                  ((self.lfsr_state >> 21) & 1) ^ 
                  ((self.lfsr_state >> 31) & 1);
        self.lfsr_state = (self.lfsr_state >> 1) | (bit << 31);
        self.lfsr_state
    }
    
    // 重置状态用于解密
    fn reset_state(&mut self) {
        self.chaos_param = self.initial_chaos;
        self.lfsr_state = self.initial_lfsr;
    }
    
    #[wasm_bindgen]
    pub fn encrypt(&mut self, img_data: &[u8]) -> Result<Vec<u8>, JsValue> {
        self.reset_state();
        
        let image = image::load_from_memory(img_data)
            .map_err(|e| JsValue::from_str(&format!("Failed to load image: {}", e)))?;
        
        let (width, height) = image.dimensions();

        let mut encrypted_image = image.clone().resize(width, height,image::imageops::FilterType::Gaussian);
        
        // 第一步：像素值扩散（使用混沌序列和LFSR）
        for y in 0..height {
            for x in 0..width {
                let pixel = image.get_pixel(x, y);
                let chaos_val = self.next_chaos();
                let lfsr_val = self.next_lfsr();
                
                // 组合混沌和LFSR输出
                let chaos_mask = (chaos_val * 255.0) as u8;
                let lfsr_mask = (lfsr_val & 0xFF) as u8;
                let combined_mask = chaos_mask ^ lfsr_mask;
                
                let encrypted_pixel = Rgba([
                    pixel[0] ^ combined_mask,
                    pixel[1] ^ ((lfsr_val >> 8) & 0xFF) as u8,
                    pixel[2] ^ ((lfsr_val >> 16) & 0xFF) as u8,
                    pixel[3], // 保持alpha通道不变
                ]);
                
                encrypted_image.put_pixel(x, y, encrypted_pixel);
            }
        }
        
        // 转换为字节数组
        let mut buffer = Vec::new();
        let mut cursor = Cursor::new(&mut buffer);
        
        encrypted_image.write_to(&mut cursor, ImageFormat::Png)
            .map_err(|e| JsValue::from_str(&format!("Failed to encode image: {}", e)))?;
        
        Ok(buffer)
    }
    
    #[wasm_bindgen]
    pub fn decrypt(&mut self, encrypted_data: &[u8]) -> Result<Vec<u8>, JsValue> {
        self.reset_state();
        
        let encrypted_image = image::load_from_memory(encrypted_data)
            .map_err(|e| JsValue::from_str(&format!("Failed to load encrypted image: {}", e)))?;
        
        let (width, height) = encrypted_image.dimensions();
        
        // 第二步：像素值逆扩散
        let mut decrypted_image = DynamicImage::new_rgba8(width, height);
        
        for y in 0..height {
            for x in 0..width {
                let pixel = encrypted_image.get_pixel(x, y);
                let chaos_val = self.next_chaos();
                let lfsr_val = self.next_lfsr();
                
                // 使用相同的掩码进行解密
                let chaos_mask = (chaos_val * 255.0) as u8;
                let lfsr_mask = (lfsr_val & 0xFF) as u8;
                let combined_mask = chaos_mask ^ lfsr_mask;
                
                let decrypted_pixel = Rgba([
                    pixel[0] ^ combined_mask,
                    pixel[1] ^ ((lfsr_val >> 8) & 0xFF) as u8,
                    pixel[2] ^ ((lfsr_val >> 16) & 0xFF) as u8,
                    pixel[3], // alpha通道保持不变
                ]);
                
                decrypted_image.put_pixel(x, y, decrypted_pixel);
            }
        }
        
        // 转换为字节数组
        let mut buffer = Vec::new();
        let mut cursor = Cursor::new(&mut buffer);
        
        decrypted_image.write_to(&mut cursor, ImageFormat::Png)
            .map_err(|e| JsValue::from_str(&format!("Failed to encode decrypted image: {}", e)))?;
        
        Ok(buffer)
    }
}