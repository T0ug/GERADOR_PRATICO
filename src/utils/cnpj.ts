export type CnpjValidationState = "empty" | "valid" | "invalid";

const CNPJ_LENGTH = 14;

export function sanitizeCnpj(value: string): string {
  return value.replace(/\D/g, "");
}

export function isValidCnpj(value: string): boolean {
  const cnpj = sanitizeCnpj(value);

  if (cnpj.length !== CNPJ_LENGTH) {
    return false;
  }

  if (/^(\d)\1+$/.test(cnpj)) {
    return false;
  }

  const digits = cnpj.split("").map(Number);
  const firstDigit = calculateCheckDigit(digits.slice(0, 12), [5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2]);
  const secondDigit = calculateCheckDigit(
    [...digits.slice(0, 12), firstDigit],
    [6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2],
  );

  return digits[12] === firstDigit && digits[13] === secondDigit;
}

export function getCnpjValidationState(value: string): CnpjValidationState {
  const sanitized = sanitizeCnpj(value);

  if (sanitized.length === 0) {
    return "empty";
  }

  return isValidCnpj(sanitized) ? "valid" : "invalid";
}

function calculateCheckDigit(numbers: number[], weights: number[]): number {
  const sum = numbers.reduce((total, number, index) => total + number * weights[index], 0);
  const remainder = sum % 11;

  return remainder < 2 ? 0 : 11 - remainder;
}
