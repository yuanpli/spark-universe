import React, { useState, useEffect } from 'react';
import axios from 'axios';
import Letter from './Letter';
import '../App.css';

const generateRandomPosition = () => {
  return {
    top: `${Math.floor(Math.random() * 80)}vh`,
    left: `${Math.floor(Math.random() * 80)}vw`,
  };
};

const LetterGrid = () => {
  const [lettersData, setLettersData] = useState([]);

  useEffect(() => {
    axios.get('http://localhost:8080/sparks/top')
      .then(response => {
        const letters = response.data.map(item => ({
          ...item,
          position: generateRandomPosition(),
        }));
        setLettersData(letters);
      })
      .catch(error => {
        console.error('Error fetching data:', error);
      });
  }, []);

  return (
    <div className="letter-container">
      <div className="star-field">
        <div className="star-layer-1"></div>
        <div className="star-layer-2"></div>
      </div>
      {lettersData.map(letter => (
        <Letter
          key={letter.id}
          text={letter.content}
          position={letter.position}
          likeCount={letter.like_count}
        />
      ))}
    </div>
  );
};

export default LetterGrid;
