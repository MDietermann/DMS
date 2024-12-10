import type { DatabaseType } from '../types/database';

export async function testDatabaseConnection(
    type: DatabaseType,
    ip: string,
    username: string,
    password: string
): Promise<boolean> {
    // In a real application, this would make an actual database connection attempt
    // For demo purposes, we'll simulate a connection test
    return new Promise((resolve) => {
        setTimeout(() => {
            // Simulate connection success if password contains "pass"
            resolve(password.includes('pass'));
        }, 1000);
    });
}

export function validateIpAddress(ip: string): boolean {
    const ipRegex = /^(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$/;
    return ipRegex.test(ip);
}
