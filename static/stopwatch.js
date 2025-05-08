class Stopwatch {
    constructor(target_id) {
      this.startTime = 0;
      this.elapsedTime = 0;
      this.isRunning = false;
      this.intervalId = null;
      this.target_id = target_id;
    }
  
    start() {
      if (this.isRunning) {
        return;
      }
  
      this.isRunning = true;
      this.startTime = Date.now() - this.elapsedTime;
      this.intervalId = setInterval(() => {
        this.elapsedTime = Date.now() - this.startTime;
        this.updateDisplay();
      }, 10);
    }
  
    stop() {
      if (!this.isRunning) {
        return;
      }
  
      this.isRunning = false;
      clearInterval(this.intervalId);
    }
  
    reset() {
       this.stop();
       this.elapsedTime = 0;
       this.updateDisplay();
    }
  
    getTime() {
        const milliseconds = Math.floor((this.elapsedTime % 1000) / 10);
        const seconds = Math.floor((this.elapsedTime / 1000) % 60);
        const minutes = Math.floor((this.elapsedTime / (1000 * 60)) % 60);
        const hours = Math.floor((this.elapsedTime / (1000 * 60 * 60)) % 24);
  
      return {
        hours,
        minutes,
        seconds,
        milliseconds,
      };
    }
  
    updateDisplay() {
        const time = this.getTime();
    //   const formattedTime = `${this.pad(time.hours)}:${this.pad(time.minutes)}:${this.pad(time.seconds)}:${this.pad(time.milliseconds)}`;
        const formattedTime = `${this.pad(time.seconds)}.${this.pad(time.milliseconds)}`
        document.getElementById(this.target_id).textContent = formattedTime;
    }
  
    pad(number) {
      return number < 10 ? '0' + number : number;
    }
  }
  
  
