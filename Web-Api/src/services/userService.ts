import { PrismaClient } from '@prisma/client';
import { hashPassword, comparePassword, generateApiKey } from '../utils/authUtils';

const prisma = new PrismaClient();

export const createUser = async (username: string, email: string, password: string) => {
    const hashedPassword = await hashPassword(password);
    const apiKey = generateApiKey();
    return prisma.user.create({
        data: {
        username,
        email,
        password: hashedPassword,
        apiKey,
        },
    });
};

export const authenticateUser = async (email: string, password: string) => {
    const user = await prisma.user.findUnique({ where: { email } });
    if (!user) return null;
    const isMatch = await comparePassword(password, user.password);
    if (!isMatch) return null;
    return user;
};

export const getApiKey = async (userId: string) => {
    const user = await prisma.user.findUnique({ where: { id: userId } });
    return user?.apiKey || null;
};

export const generateAndSaveApiKey = async (userId: string) => {
    const newApiKey = generateApiKey();
    const user = await prisma.user.update({
        where: { id: userId },
        data: { apiKey: newApiKey },
    });
    return user.apiKey;
}; 