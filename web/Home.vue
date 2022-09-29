<template lang="pug">
.container
  h1 {{name}}
</template>

<script>
export default {
  data() {
    return {
      name: "Tonnage",
    };
  },
  mounted() {
    this.checkAuth();
  },
  methods: {
    async checkAuth() {
      let token = window.localStorage.getItem("token");
      if (!token) {
        this.$router.push("/signin");
      }
    },
  },
};
</script>

<style lang="styl"></style>
