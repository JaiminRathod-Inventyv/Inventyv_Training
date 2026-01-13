// @ts-check

/**
 * Implement the classes etc. that are needed to solve the
 * exercise in this file. Do not forget to export the entities
 * you defined so they are available for the tests.
 */

export class Size{
  constructor(w=80,h=60){
    this.width = w;
    this.height = h;
  }
  resize(newWidth, newHeight){
    this.width = newWidth;
    this.height = newHeight;
  }
}
export class Position{
  constructor(x=0,y=0){
    this.x = x;
    this.y = y;
  }
  move(newX,newY){
    this.x = newX;
    this.y = newY;
  }
}

export class ProgramWindow{
  constructor() {
    this.screenSize = new Size(800, 600);
    this.size = new Size();      
    this.position = new Position();
  }
  resize(newSize) {
    const width = Math.max(newSize.width, 1);
    const height = Math.max(newSize.height, 1);
    const maxWidth = this.screenSize.width - this.position.x;
    const maxHeight = this.screenSize.height - this.position.y;
    this.size.width = Math.min(width, maxWidth);
    this.size.height = Math.min(height, maxHeight);
  }
  move(newPosition) {
    const minX = 0;
    const minY = 0;
    const maxX = this.screenSize.width - this.size.width;
    const maxY = this.screenSize.height - this.size.height;
    this.position.x = Math.min(
      Math.max(newPosition.x, minX),
      maxX
    );
    this.position.y = Math.min(
      Math.max(newPosition.y, minY),
      maxY
    );
  }
}

export function changeWindow(programWindow) {
  programWindow.resize(new Size(400, 300));
  programWindow.move(new Position(100, 150));
  return programWindow;
}