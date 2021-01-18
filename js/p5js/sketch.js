//let colors = ["#ffd659", "#f25565", "#4f7ff0", "#ac54f0", "#252525"];
let colors = ["c03221","f7f7ff","f2d0a4","545e75","3f826d","c0bda5","cc978e","f39c6b","ff3864","261447"].map(a => '#'+a);

function setup() {
	createCanvas(768, 768);
	shuffle(colors, true);
	background(255);

	circles(120);
}

function draw() {}

function circles(num){
	noStroke();
	for(let i=0; i<num; i++){
		let d = random(random(random(random(200)))) + 1;
		let x = randomGaussian(0.5, 0.25) * width - d - 1;
		let y = randomGaussian(0.5, 0.25) * height - d - 1;
		fill(random(colors));
		circle(x, y, d);
	}
}