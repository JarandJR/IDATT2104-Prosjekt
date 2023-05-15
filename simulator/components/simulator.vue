<template>
  <div class="container">
    <h3 @click="restartSimulation">Rescue simulation with drones</h3>
    <canvas id="canvas" />
  </div>
</template>

<script setup lang="js">
// Define the initial state of the simulation
let drones = [
  { x: 50, y: 50, speed: 2, rescued: false },
  { x: 100, y: 100, speed: 1.5, rescued: false },
  // Add more drones as needed
];

// Define the position of the target to be rescued
const target = { x: 200, y: 200 };

// Create a ref for the canvas element and the canvas context
const canvasRef = ref(null);
const ctxRef = ref(null);

// Define a function to update the simulation state
function updateSimulation() {
  // Perform one step of the simulation
  drones.forEach(drone => {
    if (!drone.rescued) {
      // Move the drone towards the target
      if (drone.x < target.x) {
        drone.x += drone.speed;
      } else if (drone.x > target.x) {
        drone.x -= drone.speed;
      }

      if (drone.y < target.y) {
        drone.y += drone.speed;
      } else if (drone.y > target.y) {
        drone.y -= drone.speed;
      }

      // Check if the drone has reached the target
      if (drone.x === target.x && drone.y === target.y) {
        drone.rescued = true;
      }
    }
  });
}

// Define a function to draw the current state of the simulation on the canvas
function drawSimulation() {
  // Clear the canvas
  ctxRef.value.clearRect(0, 0, canvasRef.value.width, canvasRef.value.height);

  // Draw the target
  ctxRef.value.beginPath();
  ctxRef.value.arc(target.x, target.y, 10, 0, Math.PI * 2);
  ctxRef.value.fillStyle = 'red';
  ctxRef.value.fill();
  ctxRef.value.closePath();

  // Draw the drones
  drones.forEach(drone => {
    ctxRef.value.beginPath();
    ctxRef.value.arc(drone.x, drone.y, 10, 0, Math.PI * 2);
    ctxRef.value.fillStyle = drone.rescued ? 'green' : 'blue';
    ctxRef.value.fill();
    ctxRef.value.closePath();
  });
}

// Define a function to animate the simulation
function animateSimulation() {
  // Update the simulation state
  updateSimulation();

  // Draw the simulation
  drawSimulation();

  // Check if the simulation is complete
  if (!isSimulationComplete()) {
    // If not complete, schedule the next animation frame
    requestAnimationFrame(animateSimulation);
  }
}

// Define a function to check if the simulation is complete
function isSimulationComplete() {
  // Check if all drones have been rescued
  return drones.every(drone => drone.rescued);
}

// Define a function to restart the simulation
function restartSimulation() {
  // Reset the drone positions and rescued states
  drones.forEach(drone => {
    drone.x = 50;
    drone.y = 50;
    drone.rescued = false;
  });

  // Start the simulation
  animateSimulation();
}

onMounted(() => {
  // Access the canvas element and its context after it has been mounted
  canvasRef.value = document.getElementById("canvas");
  const ctx = canvas.getContext('2d');
  
  // Assign the canvas context to the ref
  ctxRef.value = ctx;
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