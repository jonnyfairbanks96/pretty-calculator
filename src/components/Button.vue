<script>
export default {
  props: {
    text: String,
  },
}
</script>

<template>
  <button
    class="p-4 bg-gradient-to-l from-cyan-400 to-blue-200 rounded-full">{{ text }}
  </button>
</template>

<style scoped>
</style>
