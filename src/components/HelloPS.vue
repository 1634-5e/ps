<script setup>
import { ref, reactive } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

// 这个页面不需要，但是先留着
// const result = ref("")
// const photo = reactive({ pick: "" });

// function testCommand() {
//   // invoke('my_custom_command').then((message) => result.value = message)
//   invoke("my_custom_command").then((message) => {});
// }

</script>

<template>
  <div class="wholePage">
    <label class="pickBtn">
      <input
        accept="image/*"
        type="file"
        hidden="true"
        @input="submit"
      />打开图片或文件夹
    </label>
    <p class="dropbox">
      <text class="hint">
        拖拽至此
      </text>
    </p>
  </div>
</template>

<style scoped>
.wholePage {
  height: 100%;
  width: 100%;
}
.pickBtn {
  width: 21rem;
  height: 3.5rem;
  /* background-color: rgb(91, 154, 225); */
  background-color: rgb(147, 181, 207);
  color: rgb(220, 218, 215);
  cursor: pointer;
  font-size: 2rem;
  line-height: 3rem;
  display: block;
  position: absolute;
  left: calc(50% - 10.5rem);
  top: calc(12% - 1.75rem);
  text-align: center;
  border-radius: 2px;
  box-sizing: border-box;
}

.dropbox {
  position: absolute;
  text-align: center;
  width: 65%;
  height: 60%;
  left: 17.5%;
  bottom: 17%;
  border-style: dashed;
  border-width: 1px;
}

.hint{
  color: rgb(204, 204, 214);
}
</style>