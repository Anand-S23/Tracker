import { CardProps } from "../types"

export const Card: React.FC<CardProps> = ({ name, img, points, index }: CardProps) => {
    return (
        <div className={(index === 0 ? "bg-rose-400" : "bg-blue-300") + " rounded-md flex justify-between m-2 w-5/6 lg:w-1/2 mx-auto justify-self-center"}>
            <div className="flex justify-around p-2">
                <img src={img} className="w-16 h-16 lg:w-20 lg:h-20 rounded-full px-1"/>
                <p className="text-2xl grid content-center px-1">{name}</p>
            </div>

            <p className="py-2 px-10 text-2xl grid content-center">{points}</p>
        </div>
    )
}
