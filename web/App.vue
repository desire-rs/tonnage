<template lang="pug">
.container
  h1(v-text="name")
  canvas(id="myChart" width="1000")
  textarea(v-text="JSON.stringify(data, null, 2)" style="height: 500px")
</template>

<script>
import Chart from 'chart.js/auto'
import axios from 'axios';
import _ from 'lodash';
export default {
  data() {
    return {
      name: "Tonnage",
      userId: null,
      weights: [],
      data: [],
      labels: [],
      datasets: []
    };
  },
  mounted() {
    // this.chart()
    this.getChartData()
  },
  methods: {
    async getChartData() {
      let result = await axios.get('http://localhost:12306/chart');
      this.data = result.data;
      const users = _.groupBy(result.data.data.list, 'userId');
      for (const user of Object.values(users)) {
        this.labels = _.map(user, 'date');
        this.datasets.push({
          label: user[0].nickname,
          data: _.map(user, 'weight'),
          // borderColor: 'rgba(255,0,0,1)',
          // backgroundColor: 'rgba(255,0,0,0.5)',
          fill: false,
          lineTension: 0,
        })
      }

      console.log(this.labels);
      console.log(this.datasets);
      const ctx = document.getElementById('myChart').getContext('2d');
      new Chart(ctx, {
        type: 'line',
        data: {
          labels: this.labels,
          datasets: this.datasets
        },
        options: {
          responsive: false
        }
      });
    },
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
