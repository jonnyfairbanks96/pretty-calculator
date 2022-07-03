<script>
import Button from './Button.vue'

export default {
  components: {
    Button
  },
  data() {
    return {
      total: "",
      calculation: "",
    }
  },
  methods: {
    reset() {
      this.calculation = "";
    },
    addNumber(numberToAdd) {
      this.calculation += numberToAdd;
    },
    addOpp(op) {
      this.calculation += op + "\n";
    }
  },
}
</script>

<template>
  <div class="container mt-10 p-4 m-auto">
    <div class="grid grid-cols-4 gap-4 h-48">
      <h1 class="col-span-4 text-center text-xl p-5 text-cyan-200">The Pretty
        Calculator</h1>
      <!-- first row  -->
      <div
        class="p-5 bg-gradient-to-l from-cyan-400 to-blue-200 rounded-lg col-span-4">
        <span class="text-left text-black">
          {{ calculation }}
        </span>
        <span class="text-right text-black">
          {{ total }}
        </span>
      </div>
      <!-- second row  -->
      <Button @click="reset" text="AC"/>
      <Button text="&#177"/>
      <Button text="&#37;"/>
      <Button @click="addOpp('/')" text="&#247;"/>
      <Button @click="addNumber(7)" text="7"/>
      <Button @click="addNumber(8)" text="8"/>
      <Button @click="addNumber(9)" text="9"/>
      <Button text="&#215;"/>
      <Button @click="addNumber(4)" text="4"/>
      <Button @click="addNumber(5)" text="5"/>
      <Button @click="addNumber(6)" text="6"/>
      <Button text="&#45;"/>

      <Button @click="addNumber(1)" text="1"/>
      <Button @click="addNumber(2)" text="2"/>
      <Button @click="addNumber(3)" text="3"/>
      <Button text="&#43;"/>
      <!-- sixth row  -->
      <Button @click="addNumber(0)" class="col-span-2" text="0"/>
      <Button text="."/>
      <Button text="&#61;"/>
    </div>
  </div>
</template>

<style scoped>
</style>
