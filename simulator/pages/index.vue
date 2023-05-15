<template>
    <div class="wrapper">
        <p>Check backend: {{ connection }}</p>
        <Simulator/>
    </div>
  </template>
  
  <script setup lang="ts">
  import Simulator from '@/components/simulator.vue';
  import axios from 'axios'

  const connection = ref("not connected");
  const url = 'http://127.0.0.1:8080/test_connection';

  onMounted(async () => {
    const response = await axios.get(url);

    if (response.status = 200)
        connection.value = response.data;
  });
  </script>

<style scoped>

.wrapper {
    display: flex;
    
}

</style>
  