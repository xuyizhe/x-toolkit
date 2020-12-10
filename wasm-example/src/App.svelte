<script>
import wasm from '@x-toolkit/crypto-wasm';

export let name;
export let crypto_digest_sha256_result;

async function loadCryptoWasm() {
	const exports = await wasm();
	crypto_digest_sha256_result = exports.crypto_digest_sha256(
		new Uint8Array([
			104, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100
		])
	);
}
loadCryptoWasm();
</script>

<main>
	<h1>Hello {name}!</h1>
	<!-- <p>Visit the <a href="https://svelte.dev/tutorial">Svelte tutorial</a> to learn how to build Svelte apps.</p> -->

	<div class="container">
	<section >
		<h2>Crypto</h2>
		<details>
			<summary>crypto_digest_sha256(
				new Uint8Array([104, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100])
				)</summary>
			{crypto_digest_sha256_result}
		</details>
	</section>		
	</div>

</main>

<style>
main {
	text-align: center;
	padding: 1em;
	max-width: 240px;
	margin: 0 auto;
}

h1 {
	color: #ff3e00;
	text-transform: uppercase;
	font-size: 4em;
	font-weight: 100;
}

.container {
	width: 600px;
	margin: 0 auto;
	padding: 20px;
	text-align: left;	
}

@media (min-width: 640px) {
	main {
		max-width: none;
	}
}
</style>
