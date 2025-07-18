const fs = require("fs");
const path = require("path");

// Directorio raíz del monorepo
const rootDir = path.resolve(__dirname, "../");
// Carpeta donde están las apps
const appsDir = path.join(rootDir, "apps");
// Carpeta destino
const distDir = path.join(rootDir, "dist");

// Función para copiar archivos recursivamente
function copyRecursiveSync(src, dest) {
	if (!fs.existsSync(src)) return;
	const stats = fs.statSync(src);
	if (stats.isDirectory()) {
		if (!fs.existsSync(dest)) fs.mkdirSync(dest, { recursive: true });
		fs.readdirSync(src).forEach((child) =>
			copyRecursiveSync(path.join(src, child), path.join(dest, child)),
		);
	} else {
		fs.copyFileSync(src, dest);
	}
}

// Buscar apps con carpeta .next
fs.readdirSync(appsDir).forEach((appName) => {
	const appPath = path.join(appsDir, appName);
	const nextBuildPath = path.join(appPath, ".next");
	if (fs.existsSync(nextBuildPath) && fs.statSync(nextBuildPath).isDirectory()) {
		const destAppDir = path.join(distDir, "apps", appName);
		copyRecursiveSync(nextBuildPath, destAppDir);
		console.log(`Archivos de build de ${appName} copiados a ${destAppDir}`);
	}
});
