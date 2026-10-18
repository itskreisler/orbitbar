// Interactive Starfield & Clock Wallpaper Engine Script
(function () {
  const canvas = document.getElementById("particleCanvas");
  const ctx = canvas.getContext("2d");
  const clockContainer = document.getElementById("clockContainer");
  const clockTime = document.getElementById("clockTime");

  let particleCount = 150;
  let particleSpeed = 1.0;
  let showClock = true;
  let clockFormat = "24h";
  let particles = [];

  function resizeCanvas() {
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
    initParticles();
  }

  function initParticles() {
    particles = [];
    for (let i = 0; i < particleCount; i++) {
      particles.push({
        x: Math.random() * canvas.width,
        y: Math.random() * canvas.height,
        radius: Math.random() * 2 + 1,
        vx: (Math.random() - 0.5) * particleSpeed,
        vy: (Math.random() - 0.5) * particleSpeed,
        alpha: Math.random() * 0.8 + 0.2,
      });
    }
  }

  function animate() {
    ctx.clearRect(0, 0, canvas.width, canvas.height);

    for (let p of particles) {
      p.x += p.vx;
      p.y += p.vy;

      if (p.x < 0) p.x = canvas.width;
      if (p.x > canvas.width) p.x = 0;
      if (p.y < 0) p.y = canvas.height;
      if (p.y > canvas.height) p.y = 0;

      ctx.beginPath();
      ctx.arc(p.x, p.y, p.radius, 0, Math.PI * 2);
      ctx.fillStyle = `rgba(255, 255, 255, ${p.alpha})`;
      ctx.fill();
    }

    requestAnimationFrame(animate);
  }

  function updateClock() {
    if (!showClock) {
      clockContainer.style.display = "none";
      return;
    }
    clockContainer.style.display = "block";

    const now = new Date();
    let hours = now.getHours();
    const minutes = String(now.getMinutes()).padStart(2, "0");
    const seconds = String(now.getSeconds()).padStart(2, "0");

    if (clockFormat === "12h") {
      const ampm = hours >= 12 ? "PM" : "AM";
      hours = hours % 12 || 12;
      clockTime.textContent = `${hours}:${minutes}:${seconds} ${ampm}`;
    } else {
      const h = String(hours).padStart(2, "0");
      clockTime.textContent = `${h}:${minutes}:${seconds}`;
    }
  }

  // Register Wallpaper Engine Property Listener Contract
  window.wallpaperPropertyListener = {
    applyUserProperties(properties) {
      if (properties.customcolor) {
        document.documentElement.style.setProperty("--wallpaper-bg", properties.customcolor.value);
      }
      if (properties.star_count) {
        particleCount = properties.star_count.value;
        initParticles();
      }
      if (properties.particle_speed) {
        particleSpeed = properties.particle_speed.value;
        initParticles();
      }
      if (properties.show_clock !== undefined) {
        showClock = properties.show_clock.value;
        updateClock();
      }
      if (properties.clock_format) {
        clockFormat = properties.clock_format.value;
        updateClock();
      }
    },
  };

  window.addEventListener("resize", resizeCanvas);
  setInterval(updateClock, 1000);

  resizeCanvas();
  updateClock();
  animate();
})();
