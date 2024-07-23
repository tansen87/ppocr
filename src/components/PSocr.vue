<template>
  <div
    class="q-pa-md"
    style="
      display: flex;
      justify-content: space-between;
      max-width: 1500px;
      margin: 0 auto;
    "
  >
    <div class="q-gutter-y-md" style="flex-basis: 45%; max-width: 80px">
      <q-btn color="secondary" label="ocr" @click="ocr" style="width: 80px" />
    </div>
  </div>

  <q-scroll-area style="height: 380px; width: 100%">
    <div
      class="q-pa-md"
      style="
        display: flex;
        justify-content: space-between;
        max-width: 1500px;
        margin: 0 auto;
      "
    >
      <q-img
        v-if="clip"
        :src="clip"
        spinner-color="white"
        style="
          height: auto;
          max-width: 1500px;
          width: 100%;
          object-fit: contain;
        "
      />
    </div>

    <div
      class="q-pa-md"
      style="
        display: flex;
        justify-content: space-between;
        max-width: 1500px;
        margin: 0 auto;
      "
    >
      <q-input
        v-model="text"
        filled
        type="textarea"
        autogrow
        style="width: 100%"
      />
    </div>
  </q-scroll-area>
</template>

<script setup>
import { ref } from "vue";
import { useQuasar, QSpinnerGears } from "quasar";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";

const $q = useQuasar();

const text = ref();
const elapsed = ref([]);
const clip = ref();

listen("ps_time", (event) => {
  elapsed.value = event.payload;
});

// ocr
async function ocr() {
  const notif = $q.notify({
    color: "ongoing",
    icon: "ongoing",
    position: "bottom-right",
    group: false,
    timeout: 0,
    spinner: QSpinnerGears,
    message: "ocr...",
  });

  clip.value = "";
  try {
    const clipboardItems = await navigator.clipboard.read();
    const imageBlob = await clipboardItems[0].getType("image/png");
    if (imageBlob) {
      clip.value = URL.createObjectURL(imageBlob);
    } else {
      console.log("剪贴板中没有图像数据");
    }
  } catch (err) {
    console.error("无法读取剪贴板中的图像", err);
  }

  const ocrResults = await invoke("screen", {});
  const jsonData = JSON.parse(ocrResults);

  if (jsonData.data.includes("Error")) {
    notif({
      message: ocrResults,
      color: "negative",
      icon: "sentiment_very_dissatisfied",
      spinner: false,
      actions: [
        {
          label: "ok",
          color: "white",
          handler: () => {
            notifId = null;
          },
        },
      ],
    });
  } else {
    if (jsonData.code == 100) {
      text.value = jsonData.data.map((item) => item.text).join("\n");
    } else {
      text.value = jsonData.data;
    }

    notif({
      message: "ocr successful, take time: " + elapsed.value + " s",
      color: "positive",
      icon: "sentiment_satisfied_alt",
      spinner: false,
      timeout: 1500,
    });
  }
}
</script>
