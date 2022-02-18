<template>
  <div>
    <table>
      <tr class="header-row">
        <th>
          <span class="my-category"> {{ name }}</span>
        </th>
        <th>Level</th>
        <th>Max level</th>
        <th>Income</th>
      </tr>

      <tr
        v-for="[work, work_state] in thework"
        :key="work.name"
        class="no-touch"
        :class="{ mydisabled: !work_state.is_unlocked }"
        @click="work_state.is_unlocked && wasm.set_work(work.name)"
      >
        <td>
          <MyProgressBar
            v-if="work_state.next_level_percentage !== undefined"
            :value="work_state.next_level_percentage"
            :name="work.display_name"
            :selected="work.name == input.work"
          />
        </td>
        <td>
          {{ work_state.level }}
        </td>
        <td>
          {{ work_state.max_job_levels }}
        </td>
        <td>
          {{ work_state.effective_income.toFixed(1) }}
        </td>
      </tr>
    </table>
  </div>
</template>

<script>
import MyProgressBar from './MyProgressBar.vue'

export default {
  components: { MyProgressBar },
  props: ['name', 'thework', 'input', 'wasm'],
}
</script>

<style scoped>
table {
  width: 100%;
  height: 100%;
}

tr {
  height: calc(3rem + 2 px);
  display: flex;
  margin: 0 auto;
  gap: 10px;
  padding: 0.5rem;
  border-bottom: 2px solid lightgray;
  /* border-top: 2px solid lightgray; */
}

th {
  flex-grow: 1;
  flex-basis: 0;

  display: flex;
  justify-content: center;
  align-items: center;
}

td {
  flex-grow: 1;
  flex-basis: 0;
  display: flex;
  justify-content: center;
  align-items: center;
}

tr > td:first-child {
  flex-grow: 2;
}

tr > th:first-child {
  justify-content: start;
  flex-grow: 2;
}

.my-category {
  padding-left: 0.5rem;
}

/* tr:not(:last-child) { */
/*   border-top: 2px solid lightgray; */
/*   border-bottom: 1px solid lightgray; */
/* } */

tr:hover {
  cursor: pointer;
}
</style>
