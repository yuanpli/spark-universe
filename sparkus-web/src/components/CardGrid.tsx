import React, { useState, useEffect } from 'react';
import Marquee from "@/components/magicui/marquee";
import axios from 'axios';
import MsgCard from './MsgCard';

const CardGrid = () => {
  const [lettersData, setLettersData] = useState([]);

  useEffect(() => {
    axios.get('http://localhost:8080/sparks/top')
      .then(response => {
        const letters = response.data.map(item => ({
          ...item,
        }));
        setLettersData(letters);
      })
      .catch(error => {
        console.error('Error fetching data:', error);
      });
  }, []);

  return (
    <div className="relative flex h-[800px] w-full flex-col items-center justify-center overflow-hidden rounded-lg border bg-background md:shadow-xl">
      <Marquee pauseOnHover className="[--duration:20s]">
        {lettersData.map(letter => (
          <MsgCard
            key={letter.id}
            text={letter.content}
            // position={letter.position}
            postDate={letter.create_at}
            likeCount={letter.like_count}
          />
        ))}
      </Marquee>
      <div className="pointer-events-none absolute inset-y-0 left-0 w-1/3 bg-gradient-to-r from-white dark:from-background"></div>
      <div className="pointer-events-none absolute inset-y-0 right-0 w-1/3 bg-gradient-to-l from-white dark:from-background"></div>
    </div>
  );
};

export default CardGrid;
