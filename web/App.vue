<template lang="pug">
.container
  .chart
    canvas(id="myChart")
  .data
    .form
      input(type="number" placeholder="体重")
      button 提交
    table(class="data-table")
      tr
        th nickname
        th weight
        th date
      tr(v-for="item in data")
        td(v-text="item.nickname")
        td(v-text="item.weight")
        td(v-text="item.date")
</template>

<script>
import Chart from "chart.js/auto";
import axios from "axios";
import _ from "lodash";
export default {
  data() {
    return {
      name: "Tonnage",
      userId: null,
      weights: [],
      data: [],
      labels: [],
      datasets: [],
    };
  },
  mounted() {
    this.getChartData();
  },
  methods: {
    async getChartData() {
      let result = await axios.get("/chart");
      this.data = result.data.data.list.slice(0, 10);
      const users = _.groupBy(result.data.data.list, "userId");
      for (const user of Object.values(users)) {
        this.labels = _.map(user, "date");
        this.datasets.push({
          label: user[0].nickname,
          data: _.map(user, "weight"),
          borderColor: user[0].borderColor || "rgba(255,0,0,1)",
          backgroundColor: user[0].backgroundColor || "rgba(255,0,0,0.5)",
          fill: false,
          lineTension: 0,
        });
      }
      const ctx = document.getElementById("myChart").getContext("2d");
      new Chart(ctx, {
        type: "line",
        data: {
          labels: this.labels,
          datasets: this.datasets,
        },
        options: {
          responsive: false,
        },
      });
    },
  },
};
</script>

<style lang="styl">
.container
  display: flex;
  flex-direction: row;
  width: 100%;
  height: 100%;
.chart
  width: 80%;
  display: flex;
  min-height: 500px;
  justify-content: center;
  align-items: center;
  flex-grow: 1;
#myChart
  flex: 1;
.data
  display: flex;
  flex-direction: column;
  min-height: 500px;
  justify-content: center;
  align-items: center;
  flex-grow: 1;
.data-table
  font-family: Arial, Helvetica, sans-serif;
  border-collapse: collapse;
.data-table td, .data-table th
  border: 1px solid #ddd;
  padding: 8px;
.data-table tr:nth-child(even){background-color: #f2f2f2;}
.data-table tr:hover {background-color: #ddd;}
.data-table th
  padding-top: 12px;
  padding-bottom: 12px;
  text-align: left;
  background-color: cadetblue;
  color: white;

.form
  display: flex;
  justify-content: center;
  align-items: center;
</style>
