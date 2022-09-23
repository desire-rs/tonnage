<template lang="pug">
.container
  h1(v-text="name")
</template>

<script lang="ts">
export default {
  data() {
    return {
      name: "Tonnage",
      userId: null,
      weights: [],
    };
  },
  methods: {
    
  }
};
</script>

<style lang="styl"></style>
