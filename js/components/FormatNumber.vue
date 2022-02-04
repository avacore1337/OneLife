<template>
  <span> {{ printableNumber }} </span>
</template>

<script>
// Inspiration:
// https://github.com/FreddecGames/ngsc/blob/master/frontend/src/components/FormatNumber.vue
export default {
  props: ['value'],
  computed: {
    printableNumber() {
      let num = this.value
      if (num === undefined) {
        return null
      }

      if (num < 100) {
        return num.toFixed(1)
      }
      if (num < 10000) {
        return Math.floor(num).toString()
      }

      if (this.$store.state.numberFormat === 'SCIENTIFIC') {
        let exponent = 1
        while (num >= 10) {
          num /= 10
          exponent++
        }

        return `${num.toFixed(1)}e${exponent}`
      }

      const ending = ['K', 'M', 'B', 'T', 'Qa', 'Qi', 'He', 'Se', 'Oc', 'No', 'De']
      let index = -1
      while (num >= 10000 && index < ending.length - 1) {
        num /= 1000
        index++
      }

      return `${num.toPrecision(4)}${ending[index]}`
    },
  },
}
</script>
