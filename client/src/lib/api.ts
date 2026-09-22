import { PUBLIC_API_URL } from "$env/static/public";

export interface RequestChallengeResponse {
	token: string;
}

export interface ConfirmChallengeResponse {
	token: string;
}

export interface GetDepositLimitResponse {
	min_deposit_amount_in_cents: number;
	max_deposit_amount_in_cents: number;
	min_payout_amount_in_cents: number;
	max_payout_amount_in_cents: number;
}

export interface DepositRequest {
	deposit_amount_in_cents?: number;
	payout_amount_in_cents?: number;
	address: string;
}

export interface DepositResponse {
	br_code: string;
	deposit_amount_in_cents: number;
	payout_amount_in_cents: number;
	fee_amount_in_cents: number;
}

export async function requestChallenge(public_key: string): Promise<RequestChallengeResponse> {
	const response = await fetch(`${PUBLIC_API_URL}/auth/request-challenge`, {
		method: "POST",
		headers: { "Content-Type": "application/json" },
		body: JSON.stringify({
			public_key
		})
	});

	return response.json();
}

export async function confirmChallenge(token: string, signature: string): Promise<ConfirmChallengeResponse> {
	const response = await fetch(`${PUBLIC_API_URL}/auth/confirm-challenge`, {
		method: "POST",
		headers: { "Content-Type": "application/json" },
		body: JSON.stringify({
			token,
			signature
		})
	});

	return response.json();
}

export async function getDepositLimit(token: string): Promise<GetDepositLimitResponse> {
	const response = await fetch(`${PUBLIC_API_URL}/deposit/limit`, {
		method: "GET",
		headers: {
			Authorization: `Bearer ${token}`
		}
	});

	return response.json();
}

export async function createDeposit(token: string, payload: DepositRequest): Promise<DepositResponse> {
	const response = await fetch(`${PUBLIC_API_URL}/deposit`, {
		method: "POST",
		headers: {
			Authorization: `Bearer ${token}`,
			"Content-Type": "application/json"
		},
		body: JSON.stringify(payload)
	});

	return response.json();
}
