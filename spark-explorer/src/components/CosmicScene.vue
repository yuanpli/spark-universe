 <template>
  <canvas ref="sceneCanvas" class="w-full h-full"></canvas>
</template>
<!-- src/components/CosmicScene.vue


<script setup>
import { ref, onMounted, onBeforeUnmount } from 'vue'
import * as THREE from 'three'

const sceneCanvas = ref(null)
let renderer, scene, camera, animationId

onMounted(() => {
  // 初始化 Three.js 场景
  scene = new THREE.Scene()

  // 设置相机
  const aspect = window.innerWidth / window.innerHeight
  camera = new THREE.PerspectiveCamera(75, aspect, 0.1, 1000)
  camera.position.z = 5

  // 设置渲染器
  renderer = new THREE.WebGLRenderer({ canvas: sceneCanvas.value, antialias: true })
  renderer.setSize(window.innerWidth, window.innerHeight)
  renderer.setPixelRatio(window.devicePixelRatio)

  // 添加星星
  const starGeometry = new THREE.SphereGeometry(0.05, 24, 24)
  const starMaterial = new THREE.MeshBasicMaterial({ color: 0xffffff })

  for (let i = 0; i < 200; i++) {
    const star = new THREE.Mesh(starGeometry, starMaterial)
    star.position.set(
      THREE.MathUtils.randFloatSpread(100),
      THREE.MathUtils.randFloatSpread(100),
      THREE.MathUtils.randFloatSpread(100)
    )
    scene.add(star)
  }

  // 添加自适应窗口大小
  window.addEventListener('resize', onWindowResize, false)

  // 动画循环
  animate()
})

const onWindowResize = () => {
  const width = window.innerWidth
  const height = window.innerHeight
  camera.aspect = width / height
  camera.updateProjectionMatrix()
  renderer.setSize(width, height)
}

const animate = () => {
  animationId = requestAnimationFrame(animate)

  // 旋转场景以增加动态效果
  scene.rotation.x += 0.0005
  scene.rotation.y += 0.0005

  renderer.render(scene, camera)
}

onBeforeUnmount(() => {
  cancelAnimationFrame(animationId)
  window.removeEventListener('resize', onWindowResize)
  renderer.dispose()
})
</script>

<style scoped>
canvas {
  display: block;
  width: 100%;
  height: 100vh;
  background: radial-gradient(circle, #000010, #000000);
}
</style> -->
<script setup>
import { ref, onMounted, onBeforeUnmount } from 'vue'
import * as THREE from 'three'
import gsap from 'gsap'
import api from '../services/api'

import { FontLoader } from 'three/examples/jsm/loaders/FontLoader.js';
import { TextGeometry } from 'three/examples/jsm/geometries/TextGeometry.js';

const sceneCanvas = ref(null)
let renderer, scene, camera, animationId
const sparks = ref([]) // 保存从后端获取的Sparks

onMounted(async () => {
  // 获取Sparks数据
  try {
    const response = await api.get('/sparks/latest')
    console.log(response.data)
    sparks.value = response.data
  } catch (error) {
    console.error('Failed to fetch sparks:', error)
  }

  // 初始化 Three.js 场景
  scene = new THREE.Scene()
 console.log('1111111111')
  // 设置相机
  const aspect = window.innerWidth / window.innerHeight
  camera = new THREE.PerspectiveCamera(75, aspect, 0.1, 1000)
  camera.position.z = 5
  console.log('2222222')

  // 设置渲染器
  renderer = new THREE.WebGLRenderer({ canvas: sceneCanvas.value, antialias: true })

  console.log('33333')
  renderer.setSize(window.innerWidth, window.innerHeight)

  console.log('33333')
  // 添加星星
  const starGeometry = new THREE.SphereGeometry(0.05, 24, 24)
  const starMaterial = new THREE.MeshBasicMaterial({ color: 0xffffff })

  for (let i = 0; i < 200; i++) {
    const star = new THREE.Mesh(starGeometry, starMaterial)
    star.position.set(
      THREE.MathUtils.randFloatSpread(100),
      THREE.MathUtils.randFloatSpread(100),
      THREE.MathUtils.randFloatSpread(100)
    )
    scene.add(star)
  }

  // 添加行星动画
  createSparks()

  // 添加鼠标悬停效果
  addHoverEffect()

  // 开始动画循环
  animate()
})

onBeforeUnmount(() => {
  cancelAnimationFrame(animationId)
  renderer.dispose()
})

function createSparks() {
  sparks.value.forEach((spark, index) => {
    // 获取 Spark 的内容，截取前20个字符
    const content = spark.content.slice(0, 20);

    // 创建 3D 文字的几何体
    const fontLoader = new FontLoader();
    fontLoader.load('/fonts/optimer_regular.typeface.json', (font) => {  // 替换为你字体文件的路径
      const geometry = new TextGeometry(content, {
        font: font,
        size: 0.5,  // 字体大小
        height: 0.2,  // 立体效果的深度
        curveSegments: 12,
        bevelEnabled: true,
        bevelThickness: 0.03,
        bevelSize: 0.02,
        bevelOffset: 0,
        bevelSegments: 5,
      });

      const material = new THREE.MeshBasicMaterial({ color: getColorByLikes(spark.starCount) });
      const textMesh = new THREE.Mesh(geometry, material);

      // 随机初始位置，Z 轴设定为 -20
      textMesh.position.set(
        THREE.MathUtils.randFloatSpread(20), // 随机 X 轴位置
        THREE.MathUtils.randFloatSpread(20), // 随机 Y 轴位置
        -20 // 初始位置在视野之外
      );

      scene.add(textMesh);

      // 设置动画循环
      const animateText = () => {
        // 动画：文字从屏幕后方向前推进
        gsap.to(textMesh.position, {
          z: 5, // 最终位置靠近屏幕
          duration: 10 + index * 0.5, // 每个文字稍微错开时间
          ease: 'linear',
          onComplete: () => {
            textMesh.position.z = -20; // 重置位置
            animateText(); // 重新开始动画
          },
        });

        // 动画：文字逐渐放大
        gsap.to(textMesh.scale, {
          x: 1.5,
          y: 1.5,
          duration: 10 + index * 0.5,
          ease: 'linear',
        });
      };

      animateText(); // 开始动画
    });
  });
}

// 根据 like 数量生成一个明亮的颜色
function getColorByLikes(likes) {
  const hue = (likes * 25) % 360; // 根据 likes 计算颜色的色相，取值范围 [0, 360)
  const saturation = 100;
  const lightness = 50 + (50 - Math.min(likes, 50)); // 保证 lightness 不会太低，最低为50
  return `hsl(${hue}, ${saturation}%, ${lightness}%)`;
}



// function getColorByLikes(likes) {
//   const hue = likes % 360
//   const saturation = likes > 50 ? 100 : likes * 2
//   const lightness = likes > 50 ? 50 : 50 + (likes / 2)
//   return `hsl(${hue}, ${saturation}%, ${lightness}%)`
// }

function addHoverEffect() {
  const raycaster = new THREE.Raycaster()
  const mouse = new THREE.Vector2()

  function onMouseMove(event) {
    mouse.x = (event.clientX / window.innerWidth) * 2 - 1
    mouse.y = -(event.clientY / window.innerHeight) * 2 + 1

    raycaster.setFromCamera(mouse, camera)
    const intersects = raycaster.intersectObjects(scene.children)

    if (intersects.length > 0) {
      const intersectedPlanet = intersects[0].object
      gsap.to(intersectedPlanet.scale, { x: 3, y: 3, z: 3, duration: 0.5 }) // 放大选中的行星
    }
  }

  function onClick() {
    const intersects = raycaster.intersectObjects(scene.children)
    if (intersects.length > 0) {
      const intersectedPlanet = intersects[0].object
      const sparkId = intersectedPlanet.userData.id

      // 向后端发送点赞请求
      api.post(`/sparks/${sparkId}/like`).then(() => {
        console.log('Liked spark:', sparkId)
      }).catch(error => {
        console.error('Failed to like spark:', error)
      })
    }
  }

  window.addEventListener('mousemove', onMouseMove, false)
  window.addEventListener('click', onClick, false)
}

function animate() {
  animationId = requestAnimationFrame(animate)
  // 旋转场景以增加动态效果
  scene.rotation.x += 0.0005
  scene.rotation.y += 0.0005
  renderer.render(scene, camera)
}
</script>
