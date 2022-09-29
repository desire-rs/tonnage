<template lang="pug">
.container
  h1 Hello App!
  p
    router-link(to="/") Go to Home
    router-link(to="/signin") signin
router-view
</template>
