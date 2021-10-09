import React, {
  Component,
  PureComponent,
  useEffect,
  useLayoutEffect,
  useMemo,
  useState,
} from "react";
import Konva from "konva";
import { render } from "react-dom";
import { Stage, Layer, Group, Line, Rect, Ellipse } from "react-konva";
import * as wasm from "hello-wasm-pack";
import toast, { Toaster } from "react-hot-toast";
import Modal from "react-modal";
import helpImg from "./help.jpg";
import { useStrictMode } from "react-konva";
import debounce from "lodash.debounce";

useStrictMode(true)

Modal.setAppElement("#root");

function getRandomColor() {
  var letters = "0123456789ABCDEF";
  var color = "#";
  for (var i = 0; i < 6; i++) {
    color += letters[Math.floor(Math.random() * 16)];
  }
  return color;
}

function is_clockwise(points) {
  let sum = 0;
  for (let i = 0; i < points.length; i++) {
    let j = i === points.length - 1 ? 0 : i + 1;
    sum += (points[j].x - points[i].x) * (points[j].y + points[i].y);
  }
  return sum < 0;
}

document.addEventListener(
  "contextmenu",
  function (e) {
    e.preventDefault();
  },
  false
);

function Button({ onClick, children, disabled, style = {} }) {
  return (
    <button
      style={{
        margin: "0 4px",
        padding: "8px 16px",
        borderRadius: "5px",
        border: "none",
        cursor: "pointer",
        boxShadow: "0px 1px 2px rgba(126,56,0,0.5)",
        ...style,
      }}
      onClick={onClick}
      disabled={disabled}
    >
      {children}
    </button>
  );
}

function App() {
  const [points, setPoints] = useState([]);
  const [curMousePos, setCurMousePos] = useState([0, 0]);
  const [primaryPolygons, setPrimaryPolygons] = useState([]);
  const [clipPolygons, setClipPolygons] = useState([]);
  const [intersect, setIntersect] = useState([]);

  const [stage, setStage] = useState("PRIMARY");

  const [isOpen, setIsOpen] = useState(false);

  const getMousePos = (stage) => {
    return [stage.getPointerPosition().x, stage.getPointerPosition().y];
  };
  const handleClick = (event) => {
    if (stage === "MOVE") {
      return;
    }
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

  const handleClip = (primaryPolygons, clipPolygons) => {
    if (primaryPolygons.length === 0 || clipPolygons.length === 0) {
      return;
    }
    const transform = (polygons) => {
      return polygons
        .map((points) => {
          return points.map((p) => {
            return {
              x: p[0],
              y: p[1],
            };
          });
        })
        .map((points, index) => {
          if (is_clockwise(points) && index == 0) {
            return points;
          } else {
            return [...points].reverse();
          }
        });
    };

    const primary = transform(primaryPolygons);
    const clip = transform(clipPolygons);

    console.log(primary, clip)
    try {
      const res = wasm.clip(clip, primary);
      if (!res) {
        return;
      }
      const _intersect = res.map((points) => {
        return points.map((p) => {
          return [p.x, p.y];
        });
      });
      console.log(_intersect)
      setIntersect(_intersect);
    } catch (error) {
      toast.error("Something wrong happened");
      setIntersect([]);
    }
  };

  const flattenedPoints = points
    .concat(curMousePos)
    .reduce((a, b) => a.concat(b), []);
  const intersects = intersect.reduce((a, b) => a.concat(b), []);

  const handleEsc = (event) => {
    if (event.key === "Escape") setPoints([]);
  };

  useEffect(() => {
    document.addEventListener("keydown", handleEsc);
    return () => {
      document.removeEventListener("keydown", handleEsc);
    };
  }, []);

  const handleMovePrimary = (event) => {
    const x = event.target.attrs.x
    const y = event.target.attrs.y
    const deltaX = x
    const deltaY = y
    const res = primaryPolygons.map((points, index) => {
      if(index!=event.target.index){
        return  [...points]
      }
      return points.map((p) => {
        return [p[0] + deltaX, p[1] + deltaY];
      });
    });
    handleClip(res, clipPolygons)
  };

  const handleDragEnd = (event) => {
    const x = event.target.attrs.x
    const y = event.target.attrs.y
    const deltaX = x
    const deltaY = y
    const res = primaryPolygons.map((points, index) => {
      if(index!=event.target.index){
        return  [...points]
      }
      return points.map((p) => {
        return [p[0] + deltaX, p[1] + deltaY];
      });
    });
    setPrimaryPolygons(res)
    handleClip(res, clipPolygons)
  };

  const handleMoveClip = (event) => {
    const x = event.target.attrs.x
    const y = event.target.attrs.y
    const deltaX = x
    const deltaY = y
    const res = clipPolygons.map((points, index) => {
      if(index!=event.target.index - primaryPolygons.length){
        return  [...points]
      }
      return points.map((p) => {
        return [p[0] + deltaX, p[1] + deltaY];
      });
    });
    console.log("Hello")
    handleClip(primaryPolygons, res)
  };

  const handleDragEndClip = (event) => {
    const x = event.target.attrs.x
    const y = event.target.attrs.y
    const deltaX = x
    const deltaY = y
    const res = clipPolygons.map((points, index) => {
      if(index!=event.target.index  - primaryPolygons.length){
        return  [...points]
      }
      return points.map((p) => {
        return [p[0] + deltaX, p[1] + deltaY];
      });
    });
    setClipPolygons(res)
    handleClip(primaryPolygons, res)
  };



  useEffect(()=>{
    if(stage==="MOVE"){
      handleClip(primaryPolygons, clipPolygons)
    }
  }, [stage, primaryPolygons, clipPolygons])

  const renderPolygons = (polygons, color1, color2, onDragMove, onDragEnd) => {
    return polygons.map((points, index) => {
      if (index === 0) {
        return (
          <Line
            x={0}
            y={0}
            key={index}
            opacity={0.8}
            points={points.reduce((a, b) => a.concat(b), [])}
            stroke="black"
            strokeWidth={5}
            closed={true}
            draggable
            fill={color1}
            onDragMove={onDragMove}
            onDragEnd= {onDragEnd}
          />
        );
      } else {
        return (
          <Line
            key={index}
            opacity={0.8}
            points={points.reduce((a, b) => a.concat(b), [])}
            stroke="black"
            strokeWidth={5}
            closed={true}
            draggable
            fill={color2}
            onDragMove={onDragMove}
            onDragEnd= {onDragEnd}
          />
        );
      }
    });
  };

  return (
    <>
      <div
        style={{
          display: "flex",
          justifyContent: "end",
          marginTop: "12px",
          marginRight: "8px",
          alignItems: "center",
        }}
      >
        <div
          style={{ marginRight: "24px", fontSize: "16px", color: "#605f5f" }}
        >
          Tips: You can press ESC to cancel current drawing.
        </div>
        <div>
          <Button
            disabled={!(primaryPolygons.length && clipPolygons.length)}
            onClick={() => {
              handleClip(primaryPolygons, clipPolygons);
              toast.success("Clip Success!");
            }}
          >
            Clip
          </Button>
          <Button
            onClick={() => {
              setStage("MOVE");
              toast.success("Switch to move");
            }}
          >
            Move
          </Button>
          <Button
            style={{ marginRight: "8px" }}
            onClick={() => {
              setStage("PRIMARY");
              toast.success("Switch to draw primary polygon");
            }}
          >
            Plot Primary Polygon
          </Button>
          <Button
            onClick={() => {
              setStage("CLIP");
              toast.success("Switch to draw clip polygon");
            }}
          >
            Plot Clip Polygon
          </Button>
          <Button
            onClick={() => {
              setPrimaryPolygons([]);
              setClipPolygons([]);
              setIntersect([]);
              setPoints([]);
              toast.success("Clear all drawings");
            }}
          >
            Clear
          </Button>
          <Button
            onClick={() => {
              setIsOpen(true);
            }}
          >
            Help
          </Button>
        </div>
      </div>
      <Stage
        width={window.innerWidth}
        height={window.innerHeight - 100}
        onMouseDown={handleClick}
        onMouseMove={handleMouseMove}
      >
        <Layer>
          {useMemo(() => {
            return renderPolygons(primaryPolygons, "red", "white", handleMovePrimary, handleDragEnd)}, [primaryPolygons, clipPolygons])
          }
          {useMemo(() => {
            return renderPolygons(clipPolygons, "blue", "white", handleMoveClip, handleDragEndClip)}, [primaryPolygons, clipPolygons])
          }
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
          })}
        </Layer>
      </Stage>
      <Toaster position="top-center" />
      <Modal
        isOpen={isOpen}
        style={{
          content: {
            display: "flex",
            justifyContent: "center",
            alignItems: "center",
          },
        }}
        onRequestClose={() => {
          setIsOpen(false);
        }}
        contentLabel="Help"
      >
        <Button
          onClick={() => {
            setIsOpen(false);
          }}
          style={{ position: "absolute", right: "10px", top: "10px" }}
        >
          x
        </Button>
        <div>
          <p style={{ marginLeft: "20px" }}>Tips:</p>
          <ul>
            <li style={{ marginBottom: "8px" }}>
              Left click mouse to draw polygon points{" "}
            </li>
            <li style={{ marginBottom: "8px" }}>
              Right click mouse to draw the last polygon point and close the
              drawing polygon{" "}
            </li>
            <li style={{ marginBottom: "8px" }}>
              Click 'Plot Clip Polygon' to switch to draw clip
            </li>
            <li style={{ marginBottom: "8px" }}>
              Click 'Plot Primary Polygon' to draw primary polygon
            </li>
            <li style={{ marginBottom: "8px" }}>
              Click 'Clear' will clear all drawings and press 'Esc' will cancel
              current drawing{" "}
            </li>
          </ul>
          <img width="600px" height="550px" src={helpImg}></img>
        </div>
      </Modal>
    </>
  );
}

export default App;
