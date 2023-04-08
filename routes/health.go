package routers

import (
	"github.com/gin-gonic/gin"
)

// @BasePath /api/v1

// Health
// @Summary healthcheck
// @Schemes
// @Description healthcheck
// @Tags health
// @Produce json
// @Router /health [get]
func Health(c *gin.Context) {
	c.JSON(200, gin.H{
		"status": "UP", "cluster": "Edge",
	})
}
