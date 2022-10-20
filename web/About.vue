<template lang="pug">
.container
  .form
    video(src="videos/mov_bbb.mp4" controls)
</template>

  <script>
import axios from "axios";
axios.defaults.baseURL = window.location.href.split("#")[0];
export default {
  data() {
    return {
      username: "",
      password: "",
    };
  },
  methods: {
    async submit() {
      if (!this.username || !this.password) {
        alert("username or password can not empty");
      }
      const data = { username: this.username, password: this.password };
      const result = await axios.post("/signin", data);
      const { token, exp } = result.data.data;
      if (token) {
        window.localStorage.setItem("token", token);
        window.localStorage.setItem("exp", exp);
        window.localStorage.setItem("username", this.username);
      }
      this.$router.push("/");
    },
  },
};
</script>
