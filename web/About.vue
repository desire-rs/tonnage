<template lang="pug">
.container
  .form
    video(src="videos/mov_bbb.mp4" controls style="width: 600px")
</template>
<script lang="ts">
import axios from "axios";
axios.defaults.baseURL = window.location.href.split("#")[0];
export default {
  data() {
    return {};
  },
  methods: {},
};
</script>
