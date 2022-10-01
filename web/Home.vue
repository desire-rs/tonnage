<template lang="pug">
.container
  .chart
    canvas(id='myChart')
  .data
    .form
      select(name='dataType' id='dataType' @change='onDataTypeChange($event)')
        option(v-for='dataType in dataTypes' :value='dataType.value') {{  dataType.name  }}
      input(type='number' placeholder='体重' v-model='weight')
      button(@click='submit') 提交
</template>

<script>
import Chart from "chart.js/auto";
import axios from "axios";
axios.defaults.baseURL = window.location.href;
axios.defaults.headers = {
  Authorization: `Bearer ${window.localStorage.getItem("token")}`,
};
import _ from "lodash";
export default {
  data() {
    return {
      dataTypes: [
        { name: "周", value: "W" },
        { name: "月", value: "M" },
        { name: "年", value: "Y" },
      ],
      showTable: false,
      name: "Tonnage",
      user_id: null,
      weight: 0,
      userInfo: null,
      users: [],
      charts: [],
      weights: [],
      data: [],
      labels: [],
      datasets: [],
    };
  },
  mounted() {
    this.checkAuth();
  },
  methods: {
    async checkAuth() {
      let token = window.localStorage.getItem("token");
      if (!token) {
        this.$router.push("/signin");
        return;
      }
      this.loadData();
    },
    async submit() {
      let data = { user_id: this.usersInfo.user.id, weight: this.weight };
      const result = await axios.post("/weight", data);
      console.log(result.data);
    },
    async loadData() {
      const userInfo = await axios.get("/user/info");
      const chartResult = await axios.get("/chart");
      this.userInfo = userInfo.data.data;
      this.charts = chartResult.data.data.list;
      this.data = chartResult.data.data.list.slice(0, 10);
      const users = _.groupBy(chartResult.data.data.list, "user_id");
      this.labels = _.map(_.first(Object.values(users)), "date");
      const borderColor = _.get(
        _.find(this.userInfo.props, { name: "borderColor" }),
        "value",
        "rgba(255,0,0,1)"
      );
      const backgroundColor = _.get(
        _.find(this.userInfo.props, { name: "backgroundColor" }),
        "value",
        "rgba(255,0,0,1)"
      );
      for (const user of Object.values(users)) {
        this.datasets.push({
          label: this.userInfo.user.nickname,
          data: _.map(user, "weight"),
          borderColor,
          backgroundColor,
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
    onDataTypeChange(event) {
      console.log(event.target.value);
      const user_id = event.target.value;
      this.user_id = Number(user_id);
    },
  },
};
</script>

<style lang="styl" scoped>
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
