<template>
    <div class="wrapper">
        <p>Check backend: {{ connection }}</p>
        <p>{{ data }} </p>
        <Simulator/>
    </div>
  </template>
  
  <script setup lang="ts">
  import Simulator from '@/components/simulator.vue';
  import axios from 'axios';

  const connection = ref("not connected");
  const url = 'http://127.0.0.1:8080/';
  const data = ref("Error");

  onMounted(async () => {
    const response = await axios.get(url + "test_connection");

    if (response.status = 200)
        connection.value = response.data;

    const response2 = await axios.get(url + "do_step");

    if (response2.status = 200)
        data.value = response2.data;
  });
  </script>

<style scoped>

.wrapper {
    display: flex;
    
}

</style>
  