<template>
  <div class="section">
    <table>
      <tr>
        Item Queue
      </tr>
      <tr v-b-tooltip.hover.left="info_text(item)" v-for="item in item_queue" :key="item.name">
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
.section {
  width: 100%;
  margin-bottom: 1rem;
  padding: 0.5rem 0.5rem 0.5rem 1rem;
  background-color: #333c4a;
  border-radius: 1rem 1rem 1rem 1rem;
  border-top: 0.4rem solid #1a202b;
  border-left: 0.4rem solid #1a202b;
  border-right: 0.4rem solid #1a202b;
  border-bottom: 0.4rem solid #1a202b;
}

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
