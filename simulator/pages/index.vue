<template>
    <div class="wrapper">
        <p>Check backend: {{ connection }}</p>
        <Simulator/>
    </div>
  </template>
  
  <script setup lang="ts">
  import Simulator from '@/components/simulator.vue';
  import axios from 'axios';

  const connection = ref("not connected");
  const url = 'http://127.0.0.1:8080/';

  onMounted(async () => {
    const response = await axios.get(url + "test_connection");

    if (response.status = 200)
        connection.value = response.data;
  });
  </script>

<style>

html, body {
    margin: 0;
    padding: 0;
}
.wrapper {
    display: flex;
    width: 100%;
    height: 100%;
    background-image: url('@/public/background.jfif');
    background-size: cover;
    background-position: center;
}

</style>
  