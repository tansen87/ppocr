<template>
  <q-scroll-area style="height: 450px; width: 100%">
    <div
      class="q-pa-md"
      style="
        display: flex;
        justify-content: space-between;
        max-width: 1500px;
        margin: 0 auto;
      "
    >
      <!-- column 1 -->
      <div class="q-gutter-y-md" style="flex-basis: 50%; max-width: 750px">
        <q-input color="teal" v-model="data.file" label="image file" autogrow>
        </q-input>
      </div>

      <!-- column 2 -->
      <div class="q-gutter-y-md" style="flex-basis: 45%; max-width: 160px">
        <q-input>
          <template v-slot:prepend>
            <q-btn
              color="secondary"
              label="open"
              @click="openFile"
              style="width: 80px"
            />
            <q-btn
              color="secondary"
              label="ocr"
              @click="ocr"
              style="width: 80px"
            />
          </template>
        </q-input>
      </div>
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
import { ref, reactive } from "vue";
import { useQuasar, QSpinnerGears, Notify } from "quasar";
import { open } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";

const $q = useQuasar();

const text = ref();
const elapsed = ref([]);
const data = reactive({
  file: "",
  fileExtension: ["png", "jpg", "jpeg", "PNG", "JPG", "JPEG"],
});

listen("time", (event) => {
  elapsed.value = event.payload;
});

// open file
async function openFile() {
  const selected = await open({
    multiple: false,
    filters: [
      {
        name: "image",
        extensions: data.fileExtension,
      },
    ],
  });
  if (Array.isArray(selected)) {
    data.file = selected.toString();
  } else if (selected === null) {
    Notify.create({
      type: "warning",
      message: "未选择image",
      position: "bottom-right",
    });
    return;
  } else {
    data.file = selected;
  }
}

// ocr picture
async function ocr() {
  if (data.file === "") {
    Notify.create({
      type: "warning",
      message: "未选择image",
      position: "bottom-right",
    });
    return;
  }

  const notif = $q.notify({
    color: "ongoing",
    icon: "ongoing",
    position: "bottom-right",
    group: false,
    timeout: 0,
    spinner: QSpinnerGears,
    message: "ocr...",
  });

  const ocrResults = await invoke("image", {
    file: data.file,
  });
  const jsonData = JSON.parse(ocrResults);

  if (jsonData.data.includes("Error")) {
    let notifId = notif({
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
