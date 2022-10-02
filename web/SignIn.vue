<template lang="pug">
.container
  .form
    input(type="text" placeholder="username" v-model="username" class="login")
    input(type="password" placeholder="password" v-model="password" class="login")
    div
      button(type="submit" @click="submit" class="submit") 登录
</template>

<script lang="ts">
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

<style lang="styl" scoped>
.container
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 800px;
  flex-direction: row;
.form
  display: flex;
  justify-content: center;
  align-items: center;
  min-height:400px;
  flex-direction: column;
  padding 0 50px;
.login
  padding: 10px 16px;
  border-radius: 4px;
  margin: 10px;
  outline: none;
  border: 1px solid #c6c6c6;
  min-width: 260px;
.submit
  cursor: pointer;
  border: none;
  padding: 8px 24px;
  border-radius: 6px;
  background-color: cadetblue;
  color: white;
  margin 0 5px;
</style>
