<template>
  <div class="container">
    <h3 @click="startSimulation">Rescue simulation with drones</h3>
    <canvas id="canvas" />
  </div>
</template>

<script setup lang="js">
import axios from 'axios';

const url = 'http://127.0.0.1:8079/';
const droneColor = 'blue';
const droneSize = 10;

const target = ref({ x: 1350, y: 325 });
const drones = ref([]);

const canvasRef = ref(null);
const ctxRef = ref(null);

async function updateSimulation() {
  await axios.post(url + 'do_step', target.value)
  drones.value = (await axios.get(url + 'get_drones')).data
  console.log(drones.value)
}

function drawSimulation() {
  clearCanvas()
  drawTarget()
  drawDrones()
}

function drawDrones(x, y) {
  drones.value.forEach(drone => {
    drawCircle(drone.x, drone.y, droneSize, droneColor); 
  });
}

function drawCircle(x, y, size, color) {
  ctxRef.value.beginPath();
  ctxRef.value.fillStyle = 'black';
  ctxRef.value.arc(x, y, size + 2, 0, Math.PI * 2);
  ctxRef.value.fill();
  ctxRef.value.closePath();

  ctxRef.value.beginPath();
  ctxRef.value.fillStyle = color;
  ctxRef.value.arc(x, y, size, 0, Math.PI * 2);
  ctxRef.value.fill();
  ctxRef.value.closePath();
}

function drawTarget() {
  drawCircle(target.value.x, target.value.y, droneSize, 'red');
}

function clearCanvas() {
  ctxRef.value.clearRect(0, 0, canvasRef.value.width, canvasRef.value.height);
}

async function animateSimulation() {
  updateSimulation();
  drawSimulation();

  if (!(await axios.get(url + 'is_finished')).data) {
    requestAnimationFrame(animateSimulation);
  }
}

function startSimulation() {
  animateSimulation();
}

onMounted(async () => {
  canvasRef.value = document.getElementById("canvas");
  const ctx = canvas.getContext('2d');
  ctxRef.value = ctx;

  const devicePixelRatio = window.devicePixelRatio || 1;
  const backingStoreRatio =
    ctx.webkitBackingStorePixelRatio ||
    ctx.mozBackingStorePixelRatio ||
    ctx.msBackingStorePixelRatio ||
    ctx.oBackingStorePixelRatio ||
    ctx.backingStorePixelRatio ||
    1;
  const ratio = devicePixelRatio / backingStoreRatio;

  canvas.width = canvas.clientWidth * ratio;
  canvas.height = canvas.clientHeight * ratio;
  ctx.scale(ratio, ratio);

  drawTarget()
  drones.value = (await axios.get(url + 'get_drones')).data
  console.log(drones.value)
  drawDrones()
});

</script>

<style scoped>

canvas {
    margin-top: 0;
    border: solid black;
    width: 1400px;
    height: 650px;
}

.container {
  font-size: 20px;
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  text-align: center;
  align-items: center;
}

.container h3 {
  color: darkgray;
  font-family:'Franklin Gothic Medium', 'Arial Narrow', Arial, sans-serif;
  background-color: aliceblue;
  cursor: pointer;
  padding-bottom: 0;
  margin-bottom: 5px;
  box-shadow: 0px 0px 5px rgba(0, 0, 0, 0.5);
}

.container h3:hover {
  font-size: 25px;
  margin-bottom: 1px;
  color: black;
}

</style>