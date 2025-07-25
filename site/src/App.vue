<script lang="ts" setup>
import { ref, useTemplateRef } from 'vue';
import init, {Logistic} from './wasm';
import { useDark, watchDebounced } from '@vueuse/core';
import { SliderRange, SliderRoot, SliderThumb, SliderTrack } from 'reka-ui';

let ch: Logistic | null = null;
const key = ref([0.5]);
const seed = ref([1234]);
const keyInput = ref(0.5);
const seedInput = ref(1234);
const rawUrl = ref('');
const file = useTemplateRef('file');
const url = ref('');

const updateKeyFromInput = () => {
  if (keyInput.value >= 0 && keyInput.value <= 1) {
    key.value = [keyInput.value];
  }
};

const updateSeedFromInput = () => {
  if (seedInput.value >= 1) {
    seed.value = [Math.floor(seedInput.value)];
  }
};

const initLogistic = () => {
  if (ch) {
    ch = new Logistic(key.value[0], Math.floor(seed.value[0]));
  }
};

init()
.then(() => {
  ch = new Logistic(key.value[0], Math.floor(seed.value[0]));
});

watchDebounced([key, seed], () => {
  keyInput.value = key.value[0];
  seedInput.value = seed.value[0];
  initLogistic();
}, { deep: true, debounce: 200 });


const getFileUrl = (file: File) => {
  return new Promise<string>((resolve) => {
    const reader = new FileReader();
    reader.onload = () => {
      rawUrl.value = reader.result as string;
      resolve(rawUrl.value);
    }
    reader.readAsDataURL(file);
  })
}

const onSelectFile = () => {
  const files = file.value?.files;
  if (!files) {
    return;
  }
  const img = files[0];
  getFileUrl(img)
  .then((url) => {
    rawUrl.value = url;
  })
}

const getFile = (
  onGetSuccess: (file: Blob)=>void
) => {
if (!file.value){
    return;
  }
  const [img] = file.value.files ?? [];
  if (!img) {
    return;
  }
  const reader = new FileReader();
  reader.onload = () => {
    if (!ch) {
      return;
    }
    const res = reader.result;
    if (!res) {
      return;
    }
    if (typeof res === 'string'){
      return;
    }
    const ret:Uint8Array = ch.encrypt(new Uint8Array(res));
    const blob = new Blob([ret], { type: img.type });
    onGetSuccess(blob);
  }
  reader.readAsArrayBuffer(img);
}


const onSubmit = () => {
  url.value = '';
  getFile((file) => {
    url.value = URL.createObjectURL(file);
  })
}

useDark();

</script>

<template>
  <div class="min-h-screen bg-gradient-to-br from-slate-50 to-slate-100 dark:from-slate-900 dark:to-slate-800 p-4">
    <div class="container mx-auto max-w-4xl">
      <div class="text-center mb-8 pt-8">
        <h1 class="text-3xl font-bold text-slate-800 dark:text-slate-200 mb-2">
          图片混淆加密工具
        </h1>
        <p class="text-slate-600 dark:text-slate-400">
          使用 Logistic 混沌算法对图片进行加密混淆
        </p>
      </div>
      <div class="bg-white dark:bg-slate-800 rounded-2xl shadow-xl border border-slate-200 dark:border-slate-700 p-8 mb-8">
        <div class="flex flex-col items-center space-y-6">
          <div class="w-full max-w-md space-y-6">
            <div>
              <label class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-3">
                混沌参数 Key (越靠近1混沌成都越高)
              </label>
              <div class="flex items-center space-x-3">
                <input
                  v-model.number="keyInput"
                  @input="updateKeyFromInput"
                  type="number"
                  min="0"
                  max="1"
                  step="0.001"
                  class="
                    w-20 px-2 py-1 text-sm text-slate-700 dark:text-slate-300
                    bg-slate-50 dark:bg-slate-700 
                    border border-slate-300 dark:border-slate-600
                    rounded-md
                    focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent w-full
                  "
                />
              </div>
            </div>

            <div>
              <label class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-3">
                随机种子 Seed (正整数)
              </label>
              <div class="flex items-center space-x-3">
                <input
                  v-model.number="seedInput"
                  @input="updateSeedFromInput"
                  type="number"
                  min="1"
                  step="1"
                  class="
                    w-20 px-2 py-1 text-sm text-slate-700 dark:text-slate-300
                    bg-slate-50 dark:bg-slate-700 
                    border border-slate-300 dark:border-slate-600
                    rounded-md
                    focus:outline-none focus:ring-2 focus:ring-green-500 focus:border-transparent
                    w-full
                  "
                />
              </div>
            </div>
          </div>

          <!-- 文件上传区域 -->
          <div class="w-full max-w-md">
            <label class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-3">
              选择图片文件
            </label>
            <div class="relative">
              <div
                class="
                  w-full min-h-200px px-4 py-3 text-sm text-slate-600 dark:text-slate-300
                  bg-slate-50 dark:bg-slate-700 
                  border-2 border-dashed border-slate-300 dark:border-slate-600
                  rounded-xl cursor-pointer
                  transition-all duration-300
                  hover:border-blue-400 dark:hover:border-blue-500
                  hover:bg-slate-100 dark:hover:bg-slate-600 box-border
                  focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent
                "
                @click="()=>file?.click()"
              >
                <img :src="rawUrl" class="max-w-full max-h-full" v-if=rawUrl />
              </div>
              <input
                v-show=false
                type="file" 
                ref="file" 
                accept="image/*"
                @change="onSelectFile"
                class="
                  w-full px-4 py-3 text-sm text-slate-600 dark:text-slate-300
                  bg-slate-50 dark:bg-slate-700 
                  border-2 border-dashed border-slate-300 dark:border-slate-600
                  rounded-xl cursor-pointer
                  transition-all duration-300
                  hover:border-blue-400 dark:hover:border-blue-500
                  hover:bg-slate-100 dark:hover:bg-slate-600
                  focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent
                "
              />
            </div>
          </div>
          
          <!-- 操作按钮 -->
          <button
            @click="onSubmit"
            class="
              group relative px-8 py-3 text-sm font-semibold text-white
              bg-gradient-to-r from-blue-600 to-blue-700
              hover:from-blue-700 hover:to-blue-800
              dark:from-blue-500 dark:to-blue-600
              dark:hover:from-blue-600 dark:hover:to-blue-700
              rounded-xl shadow-lg hover:shadow-xl
              transform transition-all duration-200
              hover:scale-105 active:scale-95
              focus:outline-none focus:ring-4 focus:ring-blue-500/30
              disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer border-none outline-none
            "
          >
            <span class="flex items-center space-x-2">
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                      d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"/>
              </svg>
              <span>开始混淆 / 反混淆</span>
            </span>
          </button>
        </div>
      </div>

      <!-- 结果显示区域 -->
      <div v-if="url" class="bg-white dark:bg-slate-800 rounded-2xl shadow-xl border border-slate-200 dark:border-slate-700 overflow-hidden">
        <div class="p-6 border-b border-slate-200 dark:border-slate-700">
          <div class="flex items-center justify-between">
            <h3 class="text-lg font-semibold text-slate-800 dark:text-slate-200">
              加密结果
            </h3>
            <div class="flex items-center space-x-2 text-green-600 dark:text-green-400">
              <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
              </svg>
              <span class="text-sm font-medium">加密完成</span>
            </div>
          </div>
        </div>
        
        <div class="p-6">
          <div class="max-w-2xl mx-auto">
            <img 
              :src="url" 
              class="w-full h-auto rounded-lg shadow-md border border-slate-200 dark:border-slate-600" 
              alt="加密后的图片"
            />
          </div>
          
          <!-- 操作提示 -->
          <div class="mt-6 p-4 bg-blue-50 dark:bg-blue-900/30 rounded-lg border border-blue-200 dark:border-blue-700">
            <div class="flex items-start space-x-3">
              <svg class="w-5 h-5 text-blue-600 dark:text-blue-400 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clip-rule="evenodd"/>
              </svg>
              <div>
                <p class="text-sm text-blue-800 dark:text-blue-200 font-medium">提示</p>
                <p class="text-sm text-blue-700 dark:text-blue-300 mt-1">
                  图片已成功加密混淆。右键保存图片即可下载加密后的文件。
                </p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style>
html,body,#app{
  width: 100%;
  padding: 0;
  margin: 0;
}
</style>