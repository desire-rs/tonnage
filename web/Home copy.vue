<template lang="pug">
.container
    .chart
      canvas(id="myChart")
    .data
      .form
        select(name="users" id="users" @change="onUserChange($event)")
          option(v-for="user in users" :value="user.id") {{  user.nickname  }}
        input(type="number" placeholder="体重" v-model="weight")
        button(@click="submit") 提交
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
axios.defaults.baseURL = window.location.href;
import _ from "lodash";
export default {
  data() {
    return {
      name: "Tonnage",
      user_id: null,
      weight: 0,
      users: [],
      charts: [],
      weights: [],
      data: [],
      labels: [],
      datasets: [],
    };
  },
  mounted() {
    this.loadData();
  },
  methods: {
    async submit() {
      let data = { user_id: this.user_id, weight: this.weight };
      const result = await axios.post("/weight", data);
      console.log(result.data);
    },
    async loadData() {
      let usersResult = await axios.get("/user");
      let chartResult = await axios.get("/chart");
      this.charts = chartResult.data.data.list;
      this.data = chartResult.data.data.list.slice(0, 10);
      this.users = usersResult.data.data.list;
      this.user_id =
        this.users.length > 0 ? _.get(_.first(this.users), "id", null) : null;
      const users = _.groupBy(chartResult.data.data.list, "user_id");
      this.labels = _.map(_.first(Object.values(users)), "date");
      for (const user of Object.values(users)) {
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
    onUserChange(event) {
      console.log(event.target.value);
      const user_id = event.target.value;
      this.user_id = Number(user_id);
    },
    async getChartData() {
      let usersResult = await axios.get("/user");
      let chartResult = await axios.get("/chart");
      this.data = chartResult.data.data.list.slice(0, 10);
      this.users = usersResult.data.data.list;
      const users = _.groupBy(chartResult.data.data.list, "user_id");
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
  flex-direction: column;
  margin 10px 0;
select,
input
  padding: 6px 16px;
  border-radius: 4px;
  margin: 10px;
  outline: none;
  border: 1px solid #c6c6c6;
  min-width: 200px;
button
  cursor: pointer;
  border: none;
  padding: 6px 16px;
  border-radius: 4px;
  background-color: cadetblue;
  color: white;
  margin 0 5px;
</style>
