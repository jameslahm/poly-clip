import React, { Component, useEffect, useState } from "react";
import Konva from "konva";
import { render } from "react-dom";
import { Stage, Layer, Group, Line, Rect, Ellipse } from "react-konva";
import * as wasm from "hello-wasm-pack";
import toast, { Toaster } from "react-hot-toast";
function getRandomColor() {
  var letters = "0123456789ABCDEF";
  var color = "#";
  for (var i = 0; i < 6; i++) {
    color += letters[Math.floor(Math.random() * 16)];
  }
  return color;
}

document.addEventListener(
  "contextmenu",
  function (e) {
    e.preventDefault();
  },
  false
);

function Button({onClick, children}){
  return <button style={{
    margin: "0 4px",
    padding: "8px 16px",
    borderRadius: "5px",
    border: "none",
    cursor: 'pointer',
    boxShadow:"0px 1px 2px rgba(126,56,0,0.5)"
  }
  } onClick={onClick}>{children}</button>
}

function App() {
  const [points, setPoints] = useState([]);
  const [curMousePos, setCurMousePos] = useState([0, 0]);
  const [primaryPolygons, setPrimaryPolygons] = useState([]);
  const [clipPolygons, setClipPolygons] = useState([]);
  const [intersect, setIntersect] = useState([]);

  const [stage, setStage] = useState("PRIMARY");

  const getMousePos = (stage) => {
    return [stage.getPointerPosition().x, stage.getPointerPosition().y];
  };
  const handleClick = (event) => {
    const mousePos = getMousePos(event.target.getStage());

    // right click
    if (event.evt.button === 2) {
      event.evt.preventDefault();
      event.evt.stopPropagation();
      setPoints([]);

      if (stage === "PRIMARY") {
        setPrimaryPolygons([...primaryPolygons, [...points, mousePos]]);
      } else {
        setClipPolygons([...clipPolygons, [...points, mousePos]]);
      }
    } else {
      setPoints([...points, mousePos]);
    }
  };
  const handleMouseMove = (event) => {
    const stage = event.target.getStage();
    const mousePos = getMousePos(stage);

    setCurMousePos(mousePos);
  };

  const handleClip = () => {
    const primary = primaryPolygons.map((points) => {
      return points.map((p) => {
        return {
          x: p[0],
          y: p[1],
        };
      });
    });
    const clip = clipPolygons.map((points) => {
      return points.map((p) => {
        return {
          x: p[0],
          y: p[1],
        };
      });
    });
    console.log(primary, clip);
    const res = wasm.clip(clip, primary);
    if (!res) {
      return;
    }
    const _intersect = res.map((points) => {
      return points.map((p) => {
        return [p.x, p.y];
      });
    });
    console.log(_intersect);
    setIntersect(_intersect);
  };

  const flattenedPoints = points
    .concat(curMousePos)
    .reduce((a, b) => a.concat(b), []);
  const intersects = intersect.reduce((a, b) => a.concat(b), []);

  return (
    <>
      <div style={{display:"flex", justifyContent:"end"}}>
        <Button onClick={() => {
          handleClip()
          toast.success("Clip Success!")
        }}>Clip</Button>
        <Button style={{marginRight: "8px"}} onClick={()=>{
          setStage("PRIMARY")
          toast.success("Switch to draw primary polygon")
        }}>Plot Primary Polygon</Button>
        <Button onClick={()=>{
          setStage("CLIP")
          toast.success("Switch to draw clip polygon")
        }}>Plot Clip Polygon</Button>
      </div>
      <Stage
        width={window.innerWidth}
        height={window.innerHeight}
        onMouseDown={handleClick}
        onMouseMove={handleMouseMove}
      >
        <Layer>
          {primaryPolygons.map((points, index) => {
            if (index === 0) {
              return (
                <Line
                  key={index}
                  points={points.reduce((a, b) => a.concat(b), [])}
                  stroke="black"
                  strokeWidth={5}
                  closed={true}
                  draggable
                  fill={"red"}
                />
              );
            } else {
              return (
                <Line
                  key={index}
                  points={points.reduce((a, b) => a.concat(b), [])}
                  stroke="black"
                  strokeWidth={5}
                  closed={true}
                  draggable
                  fill={"white"}
                />
              );
            }
          })}
          {clipPolygons.map((points, index) => {
            if (index === 0) {
              return (
                <Line
                  key={index}
                  points={points.reduce((a, b) => a.concat(b), [])}
                  stroke="black"
                  strokeWidth={5}
                  closed={true}
                  draggable
                  fill={"blue"}
                />
              );
            } else {
              return (
                <Line
                  key={index}
                  points={points.reduce((a, b) => a.concat(b), [])}
                  stroke="black"
                  strokeWidth={5}
                  closed={true}
                  draggable
                  fill={"white"}
                />
              );
            }
          })}
          {points.map((point, index) => {
            const width = 6;
            const x = point[0] - width / 2;
            const y = point[1] - width / 2;
            return (
              <Rect
                key={index}
                x={x}
                y={y}
                width={width}
                height={width}
                fill="white"
                stroke="black"
                strokeWidth={3}
              />
            );
          })}
          {
            <Line
              points={flattenedPoints}
              stroke="black"
              strokeWidth={5}
              closed={true}
              dash={[10, 10]}
              draggable
              fill={"purple"}
            />
          }
          {intersect.map((points, index) => {
            if (index === 0) {
              return (
                <Line
                  key={index}
                  points={points.reduce((a, b) => a.concat(b), [])}
                  stroke="black"
                  strokeWidth={5}
                  closed={true}
                  draggable
                  fill={"orange"}
                />
              );
            } else {
              return (
                <Line
                  key={index}
                  points={points.reduce((a, b) => a.concat(b), [])}
                  stroke="black"
                  strokeWidth={5}
                  closed={true}
                  draggable
                  fill={"orange"}
                />
              );
            }
          })}
        </Layer>
      </Stage>
      <Toaster position="top-center" />
    </>
  );
}

export default App;
