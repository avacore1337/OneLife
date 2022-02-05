<template>
  <span> {{ prettyPrintDays }} </span>
</template>

<script>
// Inspiration:
// https://github.com/FreddecGames/ngsc/blob/master/frontend/src/components/FormatNumber.vue
export default {
  props: ['value'],
  computed: {
    prettyPrintDays() {
      let total_days = this.value
      const years = Math.floor(total_days / 365)
      const days = total_days % 365

      if (years === 0) {
        return `${days} days`
      } else if (days === 0) {
        return `${years} years`
      }
      return `${years} years and ${days.toFixed(0)} days`
    },
  },
}
</script>
