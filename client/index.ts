export const init = async () => {
	const canvas = document.getElementById("renderer")! as HTMLCanvasElement;
	canvas.width = innerWidth;
	canvas.height = innerHeight;

	const gl = canvas.getContext("webgl2");
	if (!gl) {;
		document.body.innerHTML = "This example requires WebGL 2 which is unavailable on this system.";
		throw "WebGL 2 not available";
	};

	gl.clearColor(0.0, 0.0, 0.0, 1.0);
	gl.enable(gl.BLEND);
	gl.blendFunc(gl.ONE, gl.ONE_MINUS_SRC_ALPHA);

	let fsSource = await (await fetch("./shaders/shader.fs")).text();
	let vsSource = await (await fetch("./shaders/shader.vs")).text();

	const program = create_program(gl, fsSource, vsSource);
	gl.useProgram(program);
};

const create_program = (gl: WebGL2RenderingContext, fsSource: string, vsSource: string): WebGLProgram => {;
	let vShader = gl.createShader(gl.VERTEX_SHADER)!;
	let fShader = gl.createShader(gl.FRAGMENT_SHADER)!;

	gl.shaderSource(vShader, vsSource);
	gl.compileShader(vShader);
	if (!gl.getShaderParameter(vShader, gl.COMPILE_STATUS))
		throw gl.getShaderInfoLog(vShader);

	gl.shaderSource(fShader, fsSource);
	gl.compileShader(fShader);
	if (!gl.getShaderParameter(fShader, gl.COMPILE_STATUS))
		throw gl.getShaderInfoLog(fShader);

	const program = gl.createProgram();
	gl.attachShader(program, vShader);
	gl.attachShader(program, fShader);

	gl.transformFeedbackVaryings(program, ["vPosition", "vVelocity"], gl.SEPARATE_ATTRIBS);
	gl.linkProgram(program);
	if (!gl.getProgramParameter(program, gl.LINK_STATUS))
		throw gl.getProgramInfoLog(program);

	return program;
};
