<script>
import Button from './Button.vue'
import { invoke } from '@tauri-apps/api/tauri'

export default {
  components: {
    Button
  },
  data() {
    return {
      currentOp: "",
      previousOp: "",
      operator: "",
      error: "",
    }
  },
  methods: {
    alertErr(err) {
      this.error = err;
      setTimeout(() => {this.error = ""}, 5000);
    },
    del() {
      this.currentOp = this.currentOp.slice(0, -1)
    },
    reset() {
      this.currentOp = "";
      this.previousOp = "";
      this.operator = "";
      this.total = "";
    },
    addNumber(numberToAdd) {
      if (this.previousOp != "" && this.operator == "") {
        this.reset();
        this.currentOp += numberToAdd;
      } else {
        this.currentOp += numberToAdd;
      }
    },
    addOp(op) {
      if (this.previousOp != "" && this.currentOp != "") {
        this.computeTotal(() => this.operator = op);
      } else {
        this.previousOp += this.currentOp;
        this.currentOp = "";
        this.operator = "";
        this.operator = op;
      }
    },
    computeTotal(fun = () => {}) {
      if (this.currentOp != "" && this.previousOp != "" && this.operator != "") {
        invoke(
          'compute', {
            previousOp: this.previousOp,
            currentOp: this.currentOp,
            operator: this.operator
          }
        ).then((result) => {
          this.reset();
          this.previousOp = String(result);
        }).then(fun)
        .catch((err) => {
          this.alertErr(err)
          this.reset();
        })
      } else {
        this.alertErr("Ooops try again!");
      }
    },
    percent() {
      if (this.previousOp != "") {
        this.operator = "/";
        this.currentOp = "100";
        this.computeTotal();
      } else {
        this.previousOp = this.currentOp;
        this.operator = "/";
        this.currentOp = "100";
        this.computeTotal();
      }
    }
  },
}
</script>

<template>
  <div class="container mt-10 p-4 m-auto">
    <div class="grid grid-cols-4 gap-4 h-48">
      <h1 class="col-span-4 text-center text-xl p-5 text-cyan-200">
        The Pretty Calculator
      </h1>
      <span class="text-xl col-span-4 text-center text-red-200">
        {{ error }}
      </span>
      <div
          class="grid col-span-4 m-0 p-5 bg-gradient-to-r from-cyan-400 to-blue-200 rounded-lg col-span-4">
        <span class="text-left text-grey-300">
          {{ previousOp }}
          {{ operator }}
        </span>
        <span class="text-xl text-left text-black">
          {{ currentOp }}
        </span>
      </div>
      <Button @click="reset" text="AC"/>
      <Button @click="del" text="&#8592;" />
      <Button @click="percent" text="&#37;"/>
      <Button @click="addOp('/')" text="&#247;"/>
      <Button @click="addNumber(7)" text="7"/>
      <Button @click="addNumber(8)" text="8"/>
      <Button @click="addNumber(9)" text="9"/>
      <Button @click="addOp('*')" text="&#215;"/>

      <Button @click="addNumber(4)" text="4"/>
      <Button @click="addNumber(5)" text="5"/>
      <Button @click="addNumber(6)" text="6"/>
      <Button @click="addOp('-')" text="&#45;"/>

      <Button @click="addNumber(1)" text="1"/>
      <Button @click="addNumber(2)" text="2"/>
      <Button @click="addNumber(3)" text="3"/>
      <Button @click="addOp('+')" text="&#43;"/>

      <Button @click="addNumber(0)" text="0"/>
      <Button @click="addNumber('.')" text="."/>
      <Button @click="computeTotal" class="col-span-2" text="&#61;"/>
    </div>
  </div>
</template>

<style scoped>
</style>
