'use client'

import Link from "next/link";

export default function Home() {
    return (
        <div className="w-screen h-screen mx-3 overflow-x-hidden">
            <div className="w-full h-full flex flex-col items-center justify-center mt-[-8%]">
                <p className="text-5xl font-semibold">User Behaviour API 🕵</p>
               
                <div className="flex flex-row gap-x-3">
                    <Link className="w-48 py-2 rounded-xl bg-red-400 mt-4 flex items-center justify-center hover:opacity-60" href={'/pricing'}>
                        <p className="font-medium text-xl">Pricing</p>
                    </Link>
                    <Link className="w-48 py-2 rounded-xl bg-red-400 mt-4 flex items-center justify-center hover:opacity-60" href={'/docs'}>
                        <p className="font-medium text-xl">Docs</p>
                    </Link>
                </div>
                
            </div>
        </div>
    );
}
