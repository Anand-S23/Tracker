import { useState, useEffect } from 'react';
import { Player, Option } from "./types"
import { Card } from "./components/Card";
import Select from 'react-select';

const options: Option[] = [
    { value: 0, label: 'Anand' },
    { value: 1, label: 'Corey' },
    { value: 2, label: 'Jake' },
    { value: 3, label: 'Nicholas' },
    { value: 4, label: 'Nikhil' },
];

const apiEndpoint = "http://localhost:3000/api/tracker";

export default function App() {
    const [selectedOption, setSelectedOption] = useState<Option | null>(null);
    const [cards, setCards] = useState<Player[]>([]);

    const loadData = async (url: string) => {
        await fetch(url)
            .then(async (response) => {
                let players: Player[] = await response.json();
                setCards(players);
            })
            .catch((err) => console.log(err))
    }

    useEffect(() => {
        console.log("Called");
        loadData(apiEndpoint);
    }, [cards.length]);

    const submitWin = async () => {
        if (!selectedOption) return;
        if (selectedOption.value < 0 || selectedOption.value > 4) return;

        await fetch(apiEndpoint, {
            method: 'POST',
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({"value" : selectedOption.value})
        });

        console.log(selectedOption.value);
        setCards([]);
    }

    return (
        <>
            <h1 className="text-4xl font-semibold my-4 text-center"> 
                Tracker 
            </h1>

            <div className="flex justify-center">
                <Select
                    className="w-1/2 lg:w-1/4 px-3 grid content-center"
                    defaultValue={selectedOption}
                    onChange={setSelectedOption}
                    options={options}
                />

                <button 
                    className="w-12 h-12 bg-rose-500 hover:bg-rose-400 rounded-full text-white text-4xl px-3 my-3"
                    onClick={() => submitWin()}
                >
                    +
                </button>
            </div>

            <div>
                { cards.length !== 0 && cards.map((card, index) => {
                    return <Card 
                        key={index}
                        index={index}
                        name={card.name}
                        img={card.img}
                        points={card.points}
                    />
                })}
            </div>
        </>
    );
}

