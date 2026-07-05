import init, { generate_qr } from "./pkg/qr_generator.js";

const input = document.querySelector("#qr-input");
const foreground = document.querySelector("#foreground");
const background = document.querySelector("#background");
const transparent = document.querySelector("#transparent");
const styleSelect = document.querySelector("#style");
const status = document.querySelector("#status");
const canvas = document.querySelector("#qr-canvas");
const ctx = canvas.getContext("2d");

const downloadPng = document.querySelector("#download-png");
const downloadJpeg = document.querySelector("#download-jpeg");

let latestQr = null;

function clearCanvas() {
  ctx.clearRect(0, 0, canvas.width, canvas.height);
}

function drawRoundedRect(x, y, width, height, radius) {
  const safeRadius = Math.min(radius, width / 2, height / 2);

  ctx.beginPath();
  ctx.moveTo(x + safeRadius, y);
  ctx.lineTo(x + width - safeRadius, y);
  ctx.quadraticCurveTo(x + width, y, x + width, y + safeRadius);
  ctx.lineTo(x + width, y + height - safeRadius);
  ctx.quadraticCurveTo(x + width, y + height, x + width - safeRadius, y + height);
  ctx.lineTo(x + safeRadius, y + height);
  ctx.quadraticCurveTo(x, y + height, x, y + height - safeRadius);
  ctx.lineTo(x, y + safeRadius);
  ctx.quadraticCurveTo(x, y, x + safeRadius, y);
  ctx.closePath();
  ctx.fill();
}

function drawQr(qr, options = {}) {
  const forceBackground = options.forceBackground ?? false;

  clearCanvas();

  const quietZone = 4;
  const drawableModules = qr.size + quietZone * 2;
  const moduleSize = Math.floor(canvas.width / drawableModules);
  const offset = Math.floor((canvas.width - moduleSize * drawableModules) / 2);

  if (!transparent.checked || forceBackground) {
    ctx.fillStyle = background.value;
    ctx.fillRect(0, 0, canvas.width, canvas.height);
  }

  ctx.fillStyle = foreground.value;

  for (const module of qr.modules) {
    const x = offset + (module.x + quietZone) * moduleSize;
    const y = offset + (module.y + quietZone) * moduleSize;

    if (styleSelect.value === "dot") {
      ctx.beginPath();
      ctx.arc(
        x + moduleSize / 2,
        y + moduleSize / 2,
        moduleSize * 0.42,
        0,
        Math.PI * 2
      );
      ctx.fill();
    } else if (styleSelect.value === "rounded") {
      drawRoundedRect(x, y, moduleSize, moduleSize, moduleSize * 0.25);
    } else {
      ctx.fillRect(x, y, moduleSize, moduleSize);
    }
  }
}

function render() {
  try {
    const value = input.value.trim();
    latestQr = generate_qr(value);
    status.textContent = "";
    drawQr(latestQr);
  } catch (error) {
    latestQr = null;
    clearCanvas();
    status.textContent = error instanceof Error ? error.message : String(error);
  }
}

function download(format) {
  if (!latestQr) {
    return;
  }

  const originalImage = ctx.getImageData(0, 0, canvas.width, canvas.height);

  if (format === "jpeg") {
    drawQr(latestQr, { forceBackground: true });
  }

  const mimeType = format === "jpeg" ? "image/jpeg" : "image/png";
  const extension = format === "jpeg" ? "jpg" : "png";
  const url = canvas.toDataURL(mimeType, 0.92);

  const link = document.createElement("a");
  link.href = url;
  link.download = `qr-code.${extension}`;
  link.click();

  if (format === "jpeg") {
    ctx.putImageData(originalImage, 0, 0);
  }
}

await init();

[input, foreground, background, transparent, styleSelect].forEach((element) => {
  element.addEventListener("input", render);
  element.addEventListener("change", render);
});

downloadPng.addEventListener("click", () => download("png"));
downloadJpeg.addEventListener("click", () => download("jpeg"));

render();