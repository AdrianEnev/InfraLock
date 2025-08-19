'use client'
import getUserAccountInfo from "@src/hooks/users/getUserAccountInfo";
import { UserAccountInfo } from "@interfaces/UserInterfaces";
import { useGlobalContext } from "@src/components/GlobalContext";
import { useEffect, useState } from "react";
import convertDateToString from "@src/hooks/convertDateToString";

function AccountPage() {
    const [accountInfo, setAccountInfo] = useState<UserAccountInfo | null>(null);
    const fetchAccountInfo = async () => {
        const accountInfo = await getUserAccountInfo() as UserAccountInfo;
        console.log(accountInfo)
        setAccountInfo(accountInfo);
    }
    useEffect(() => {
        fetchAccountInfo();
    }, [])
    
    const { logOut } = useGlobalContext();

    return (
        <div className="py-[3%] px-[5%]">
            <p className="font-semibold text-3xl">Account</p>
            <div className="w-full h-[2px] bg-blue-400 my-1 max-w-[30%]"></div>

            <div className="flex flex-col gap-y-3 max-w-[30%] mt-2">
                <div className="flex flex-row justify-between">
                    <p className="text-xl font-semibold">Email</p>
                    <p className="text-xl">{accountInfo?.email || 'Error'}</p>
                </div>
                <div className="flex flex-row justify-between">
                    <p className="text-xl font-semibold">Username</p>
                    <p className="text-xl">{accountInfo?.username || 'Error'}</p>
                </div>
                <div className="flex flex-row justify-between">
                    <p className="text-xl font-semibold">Registration Date</p>
                    <p className="text-xl">{convertDateToString(accountInfo?.createdAt) || 'Error'}</p>
                </div>

                {/* API key is temporarily visible fully for development purposes */}
                <div className="flex flex-row justify-between">
                    <p className="text-xl font-semibold">API Key</p>
                    <p className="text-xl max-w-1/2 truncate overflow-ellipsis overflow-hidden">{accountInfo?.apiKey || 'Error'}</p>
                </div>
                <div className="flex flex-row justify-between">
                    <p className="text-xl font-semibold">Role</p>
                    <p className="text-xl">{accountInfo?.role || 'Error'}</p>
                </div>
            </div>

            <button className="w-32 lg:w-44 h-10 rounded-md bg-blue-400 hover:opacity-60 mt-3"
                onClick={() => {
                    logOut();
                }}
            >
                <p className="text-lg font-semibold text-white">Log Out</p>
            </button>

        </div>
    )
}

export default AccountPage