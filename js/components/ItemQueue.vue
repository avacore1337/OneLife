<template>
  <div class="section">
    <table>
      <tr>
        Item Queue
      </tr>
      <tr
        v-for="item in item_queue"
        :key="item.name"
        v-b-tooltip.hover.left="info_text(item)"
        class="no-select"
      >
        <td @click="wasm.dequeue_item(item.name)">
          <span>{{ item.display_name }} </span>
        </td>
      </tr>
    </table>
  </div>
</template>

<script>
export default {
  props: ['item_queue', 'wasm'],
  methods: {
    info_text(item) {
      return item.display_name
    },
  },
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
