<template lang="pug">
.container
  canvas(id="myChart" width="1000")
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
    this.getChartData()
  },
  methods: {
    async getChartData() {
      let result = await axios.get('/chart');
      this.data = result.data;
      const users = _.groupBy(result.data.data.list, 'userId');
      for (const user of Object.values(users)) {
        this.labels = _.map(user, 'date');
        this.datasets.push({
          label: user[0].nickname,
          data: _.map(user, 'weight'),
          borderColor: user[0].borderColor || 'rgba(255,0,0,1)',
          backgroundColor: user[0].backgroundColor || 'rgba(255,0,0,0.5)',
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
  }
};
</script>

<style lang="styl">
  .container
    display: flex;
    flex-direction: column
</style>
