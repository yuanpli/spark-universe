import React from 'react';
import './Letter.css';

const Letter = ({ text, position, likeCount }) => {
  return (
    <div className="letter" style={position}>
      <div className="glow"></div>
      <div className="letter-content">
        <p>{text}</p>
        <span>❤️ {likeCount}</span>
      </div>
    </div>
  );
};

export default Letter;
