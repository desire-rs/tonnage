<template lang="pug">
.container
  h1(v-text="name")
  canvas(id="myChart" width="1000")
</template>

<script>
import Chart from 'chart.js/auto'

export default {
  data() {
    return {
      name: "Tonnage",
      userId: null,
      weights: [],
    };
  },
  mounted() {
    this.chart()
  },
  methods: {
    chart() {
      const ctx = document.getElementById('myChart').getContext('2d');;
      new Chart(ctx, {
        type: 'line',
        data: {
          labels: ['一月', '二月', '三月', '四月', '五月', '六月', '七月', '八月', '九月', '十月', '十一月', '十二月'],
          datasets: [
            {
              label: '红线',
              data: [65, 21, 34, 67, 56, 12, 54, 78, 90, 10, 12, 76],
              borderColor: 'rgba(255,0,0,1)',
              backgroundColor: 'rgba(255,0,0,0.5)',
              fill: false,
              lineTension: 0,
            },
            {
              label: '蓝线',
              data: [34, 80, 29, 76, 23, 89, 12, 67, 77, 12, 94, 45],
              borderColor: 'rgba(75,193,193,1)',
              backgroundColor: 'rgba(75,193,193,1)',
              fill: false,
              lineTension: 0,
            }
          ],
        },
        options: {
          responsive: false
        }
      });
    }
  }
};
</script>

<style lang="styl">
  .container
    display: flex;
    flex-direction: column
</style>
