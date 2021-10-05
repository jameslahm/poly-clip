import React, { Component, useState } from "react";
import Konva from "konva";
import { render } from "react-dom";
import { Stage, Layer, Group, Line, Rect } from "react-konva";
import * as wasm from "hello-wasm-pack";
function getRandomColor() {
  var letters = '0123456789ABCDEF';
  var color = '#';
  for (var i = 0; i < 6; i++) {
    color += letters[Math.floor(Math.random() * 16)];
  }
  return color;
}

function App() {
  const [points, setPoints] = useState([]);
  const [curMousePos, setCurMousePos] = useState([0, 0]);
  const [isMouseOverStartPoint, setIsMouseOverStartPoint] = useState(false);
  const [polygons, setPolygons] = useState([]);
	const [intersect, setIntersect] = useState([])

  const getMousePos = (stage) => {
    return [stage.getPointerPosition().x, stage.getPointerPosition().y];
  };
  const handleClick = (event) => {
    const stage = event.target.getStage();
    const mousePos = getMousePos(stage);

    if (isMouseOverStartPoint && points.length >= 3) {
      setPoints([]);
			setPolygons([...polygons, [...points]])
			if(polygons.length===1){
				const getVertex = (points) => {
					return points.map(p=>{
						return {
							x: p[0],
							y: p[1]
						}
					})
				}
				let res = wasm.clip(getVertex(polygons[0]),getVertex(points))
				console.log(res)
				res = res[0].map(p=>{
					return [p.x,p.y]
				});
				setIntersect(res)
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
  const handleMouseOverStartPoint = (event) => {
    event.target.scale({ x: 2, y: 2 });
    setIsMouseOverStartPoint(true);
  };
  const handleMouseOutStartPoint = (event) => {
    event.target.scale({ x: 1, y: 1 });
    setIsMouseOverStartPoint(false);
  };

  const handleDragMovePoint = (event) => {
    const points = this.state.points;
    const index = event.target.index - 1;
    const pos = [event.target.attrs.x, event.target.attrs.y];
    setPoints([...points.slice(0, index), pos, ...points.slice(index + 1)]);
  };

	const renderPoints = (points, flattenedPoints, index, closed, color) =>{
		return <>
			<Line
				key={index}
				points={flattenedPoints}
				stroke="black"
				strokeWidth={5}
				closed={true}
				draggable
				fill={color}
			/>
		{
			points.map((point, index) => {
				const width = 6;
				const x = point[0] - width / 2;
				const y = point[1] - width / 2;
				const startPointAttr =
					index === 0
						? {
								hitStrokeWidth: 12,
								onMouseOver: handleMouseOverStartPoint,
								onMouseOut: handleMouseOutStartPoint,
							}
						: null;
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
						onDragMove={handleDragMovePoint}
						{...startPointAttr}
					/>
				);
			})
		}
		</>
	}

	const flattenedPoints = points.concat(curMousePos)
    .reduce((a, b) => a.concat(b), []);
	const intersects = intersect.reduce((a,b)=>a.concat(b),[])

  return (
    <Stage
      width={window.innerWidth}
      height={window.innerHeight}
      onMouseDown={handleClick}
      onMouseMove={handleMouseMove}
    >
      <Layer>
        {polygons.map((points,index) => {
          const flattenedPoints = points
            .reduce((a, b) => a.concat(b), []);
					return renderPoints(points, flattenedPoints, index, true, "red")
        })}
				{
					renderPoints(points, flattenedPoints, -1, false, "red")
				}
				{
					renderPoints(intersect, intersects, -2, false, "blue")
				}
      </Layer>
    </Stage>
  );
}

export default App;
