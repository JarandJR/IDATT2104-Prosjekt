<template>
  <div class="container">
    <h3 @click="startSimulation">Rescue simulation with drones</h3>
    <canvas id="canvas" />
  </div>
</template>

<script setup lang="js">
import axios from 'axios';

const url = 'http://127.0.0.1:8080/';
const droneColor = 'blue';

let testDrones = [
  { x: 50, y: 50, speed: 5, rescued: false },
  { x: 104, y: 100, speed: 10, rescued: false },
  { x: 58 + 60, y: 150, speed: 5, rescued: false },
  { x: 100 + 100, y: 200, speed: 8, rescued: false },
  { x: 50 + 72, y: 300, speed: 5, rescued: false },
  { x: 100 + 123, y: 400, speed: 7, rescued: false },
  { x: 50 + 117, y: 100, speed: 5, rescued: false },
  { x: 100 + 521, y: 150, speed: 6, rescued: false },
];

const target = ref({ x: 1350, y: 325 });
const drones = ref([]);
const canvasRef = ref(null);
const ctxRef = ref(null);

async function updateSimulation() {
  /*const response = await axios.get(url + 'do_step');
  if (response.status = 200) {
    drones.value = (await axios.get(url + 'get_drones')).data
    console.log(drones.value)
  }*/
  testDrones.forEach(drone => {
    if (!drone.rescued) {
      // Move the drone towards the target
      if (target.value.x - drone.x > target.value.y - drone.y) {
        if (drone.x < target.value.x) {
          drone.x += drone.speed;
        }
        if (drone.x > target.value.x) {
          drone.x -= drone.speed;
        }
      }

      else {
        if (drone.y < target.value.y) {
          drone.y += drone.speed;
        }
        if (drone.y > target.value.y) {
          drone.y -= drone.speed;
        }
      }

      // Check if the drone has reached the target
      if (drone.x === target.value.x && drone.y === target.value.y) {
        drone.rescued = true;
      }
    }
  });
  /*const is_finished = await (axios.get(url + 'is_finished')).data;
      if (is_finished) {
        console.log("finished")
        return
      }*/
}

function drawSimulation() {
  clearCanvas()
  drawTarget()
  drawDrones()
}

function drawDrones(x, y) {
  testDrones.forEach(drone => {
    drawCircle(drone.x, drone.y, 10, 'blue'); 
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
  drawCircle(target.value.x, target.value.y, 10, 'red');
}

function clearCanvas() {
  ctxRef.value.clearRect(0, 0, canvasRef.value.width, canvasRef.value.height);
}

function animateSimulation() {
  updateSimulation();
  drawSimulation();

  if (!isSimulationComplete()) {
    requestAnimationFrame(animateSimulation);
  }
}

function isSimulationComplete() {
  return testDrones.every(drone => drone.rescued);
}

function startSimulation() {
  animateSimulation();
}

onMounted(() => {
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