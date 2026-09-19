<div class="flex flex-col items-center gap-4 text-[#f7931a]">
	<div class="relative h-44 w-36">
		<div class="absolute -top-16 left-0 h-49.5 w-full overflow-hidden">
			<div class="coin absolute top-16 left-1/2">
				<div
					class="grid h-16 w-16 animate-[spin_1.6s_linear_infinite] place-items-center rounded-full bg-[#f7931a] text-4xl font-bold text-black shadow-[0_0_8px_rgba(247,147,26,.28),0_0_20px_rgba(247,147,26,.14)]"
				>
					₿
				</div>
			</div>
		</div>

		<div class="absolute top-33 left-1/2 w-24 -translate-x-1/2">
			<div class="slot h-2.25 w-full rounded-full bg-zinc-700"></div>
		</div>
	</div>
</div>

<style>
	.coin {
		animation: coin 3s infinite;
		will-change: transform;
	}

	.slot {
		animation: slot 3s infinite;
		transform-origin: center;
		will-change: transform, box-shadow;
	}

	@keyframes coin {
		0% {
			transform: translate(-50%, -50px);
			animation-timing-function: cubic-bezier(0.45, 0.05, 0.9, 0.55);
		}

		35% {
			transform: translate(-50%, 145px);
		}

		55% {
			transform: translate(-50%, 145px);
			animation-timing-function: cubic-bezier(0.12, 0.75, 0.25, 1);
		}

		100% {
			transform: translate(-50%, -50px);
		}
	}

	@keyframes slot {
		0%,
		27%,
		62%,
		100% {
			transform: scaleX(1) scaleY(1);
			box-shadow: 0 0 0 rgba(247, 147, 26, 0);
		}

		32% {
			transform: scaleX(1.08) scaleY(1.18);
			box-shadow: 0 0 0 rgba(247, 147, 26, 0);
		}

		38%,
		52% {
			transform: scaleX(1.03) scaleY(1.08);
			box-shadow:
				0 -2px 6px rgba(247, 147, 26, 0.2),
				0 -4px 12px rgba(247, 147, 26, 0.07);
		}

		56% {
			transform: scaleX(1.12) scaleY(1.22);
			box-shadow:
				0 -2px 7px rgba(247, 147, 26, 0.16),
				0 -4px 14px rgba(247, 147, 26, 0.05);
		}
	}
</style>
