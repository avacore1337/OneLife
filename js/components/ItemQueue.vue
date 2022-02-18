<template>
  <div>
    <h4 class="section-header">Item Queue</h4>
    <div class="section" style="min-height: 20rem">
      <div
        v-for="item in item_queue"
        :key="item.name"
        v-b-tooltip.hover.left="item.description"
        class="no-select"
        @click="wasm.dequeue_item(item.name)"
      >
        <span>{{ item.display_name }} </span>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  props: ['item_queue', 'wasm'],
  methods: {},
}
</script>

<style scoped>
table {
  width: 100%;
  height: 100%;
}
tr:hover:not(:first-child) {
  cursor: pointer;
}

tr:not(:last-child) {
  border-bottom: 2px solid lightgray;
}

tr {
  height: 3rem;
}

td {
  width: 100%;
}
</style>
